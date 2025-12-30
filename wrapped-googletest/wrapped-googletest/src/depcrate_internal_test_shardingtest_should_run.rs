// Generated macro for test_should_run (function)
macro_rules! Depcrate_internal_test_shardingtest_should_run {
() => {
// Module: crate::internal::test_sharding
// Provides: {"test_should_run"}
// Dependencies: {}
pub fn test_should_run (test_case_hash : u64) -> bool { SHARDING . with (| sharding_cell | { sharding_cell . get_or_init (Sharding :: from_environment) . test_should_run (test_case_hash) }) }
};
}
