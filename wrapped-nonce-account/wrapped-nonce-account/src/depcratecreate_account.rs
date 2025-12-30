// Generated macro for create_account (function)
macro_rules! Depcratecreate_account {
() => {
// Module: crate
// Provides: {"create_account"}
// Dependencies: {}
pub fn create_account (lamports : u64) -> RefCell < AccountSharedData > { RefCell :: new (AccountSharedData :: new_data_with_space (lamports , & Versions :: new (State :: Uninitialized) , State :: size () , & system_program :: id () ,) . expect ("nonce_account") ,) }
};
}
