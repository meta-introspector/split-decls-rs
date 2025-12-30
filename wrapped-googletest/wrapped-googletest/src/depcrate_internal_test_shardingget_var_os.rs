// Generated macro for get_var_os (function)
macro_rules! Depcrate_internal_test_shardingget_var_os {
() => {
// Module: crate::internal::test_sharding
// Provides: {"get_var_os"}
// Dependencies: {}
fn get_var_os (keys : & [& str]) -> Option < OsString > { for key in keys { if let Some (value) = var_os (OsStr :: new (key)) { return Some (value) ; } } None }
};
}
