// Generated macro for create_account_for_test (function)
macro_rules! Depcratecreate_account_for_test {
() => {
// Module: crate
// Provides: {"create_account_for_test"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn create_account_for_test < S : SysvarSerialize > (sysvar : & S) -> Account { create_account_with_fields (sysvar , DUMMY_INHERITABLE_ACCOUNT_FIELDS) }
};
}
