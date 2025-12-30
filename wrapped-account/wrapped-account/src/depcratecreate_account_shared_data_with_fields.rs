// Generated macro for create_account_shared_data_with_fields (function)
macro_rules! Depcratecreate_account_shared_data_with_fields {
() => {
// Module: crate
// Provides: {"create_account_shared_data_with_fields"}
// Dependencies: {}
# [cfg (feature = "bincode")] # [doc = " Create an `Account` from a `Sysvar`."] pub fn create_account_shared_data_with_fields < S : SysvarSerialize > (sysvar : & S , fields : InheritableAccountFields ,) -> AccountSharedData { AccountSharedData :: from (create_account_with_fields (sysvar , fields)) }
};
}
