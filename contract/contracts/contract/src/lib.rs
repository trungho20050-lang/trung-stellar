#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, token};

#[contract]
pub struct RemittanceContract;

#[contractimpl]
impl RemittanceContract {
    /// Chuyển tiền với phí 3%
    /// sender: Người gửi
    /// token_address: Địa chỉ của Token (ví dụ USDC)
    /// amount: Số lượng tiền gửi
    /// bank_info: Thông tin ngân hàng tại VN (dạng String)
    pub fn send_money(env: Env, sender: Address, token_address: Address, amount: i128, bank_info: Symbol) {
        // 1. Xác thực người gửi (Bắt buộc phải có chữ ký của người gửi)
        sender.require_auth();

        // 2. Tính toán phí 3%
        let fee = (amount * 3) / 100;
        let final_amount = amount - fee;

        // 3. Khởi tạo client để tương tác với Token (ví dụ USDC)
        let client = token::Client::new(&env, &token_address);

        // 4. Chuyển tiền từ người gửi vào Contract (hoặc ví tổng của bạn)
        // Ở đây ta chuyển toàn bộ `amount` vào Contract
        client.transfer(&sender, &env.current_contract_address(), &amount);

        // 5. Phát sự kiện (Event) để Backend ở VN lắng nghe
        // Event bao gồm: Người gửi, Số tiền thực nhận, Phí, và Thông tin ngân hàng
        env.events().publish(
            (symbol_short!("REMIT"), sender),
            (final_amount, fee, bank_info)
        );
    }

    // Hàm để Trung rút tiền từ Contract về ví quản trị
    pub fn withdraw(env: Env, token_address: Address, to: Address, amount: i128) {
        // Chỗ này Trung nên thêm logic kiểm tra quyền Admin (Access Control)
        let client = token::Client::new(&env, &token_address);
        client.transfer(&env.current_contract_address(), &to, &amount);
    }
}
stellar contract invoke \
  --id CA077ZKPGR4SUSUOWKRJKG67WYLTBGH0BOAYH5TU7W5PQ37BOHC6JG3C \
  --source-account student \
  --network testnet \
  -- \
  send_money \
  --sender GARSZYFF3V3IDR3J3RTBRCKOBNNZRCAP3HUHGO4GT3EIP54JOFI7LSDJ \
  --token_address CDLZ67OYY6AX3SVPNDST74L3H7A7G7T6SAB6S7YJTXSFSZTSNCYCOEUO \
  --amount 5000000000 \
  --bank_info "VCB"