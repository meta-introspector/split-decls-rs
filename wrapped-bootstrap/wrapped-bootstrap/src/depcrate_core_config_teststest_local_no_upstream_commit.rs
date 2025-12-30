// Generated macro for test_local_no_upstream_commit (function)
macro_rules! Depcrate_core_config_teststest_local_no_upstream_commit {
() => {
// Module: crate::core::config::tests
// Provides: {"test_local_no_upstream_commit"}
// Dependencies: {}
# [test] fn test_local_no_upstream_commit () { git_test (| ctx | { let src = ctx . check_modifications (& ["c" , "d"] , CiEnv :: None) ; assert_eq ! (src , PathFreshness :: MissingUpstream) ; }) ; }
};
}
