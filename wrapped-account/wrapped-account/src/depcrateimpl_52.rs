// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
# [doc = " Return the information required to construct an `AccountInfo`.  Used by the"] # [doc = " `AccountInfo` conversion implementations."] impl solana_account_info :: Account for Account { fn get (& mut self) -> (& mut u64 , & mut [u8] , & Pubkey , bool) { (& mut self . lamports , & mut self . data , & self . owner , self . executable ,) } }
};
}
