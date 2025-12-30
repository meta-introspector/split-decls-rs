// Generated macro for get_var (function)
macro_rules! Depcrate_internal_test_shardingget_var {
() => {
// Module: crate::internal::test_sharding
// Provides: {"get_var"}
// Dependencies: {}
fn get_var (keys : & [& str]) -> Option < String > { for key in keys { if let Ok (value) = var (OsStr :: new (key)) { return Some (value) ; } } None }
};
}
