// Generated macro for test_local_changes_in_head_upstream (function)
macro_rules! Depcrate_core_config_teststest_local_changes_in_head_upstream {
() => {
// Module: crate::core::config::tests
// Provides: {"test_local_changes_in_head_upstream"}
// Dependencies: {}
# [test] fn test_local_changes_in_head_upstream () { git_test (| ctx | { let sha = ctx . create_upstream_merge (& ["a"]) ; assert_eq ! (ctx . check_modifications (& ["a" , "d"] , CiEnv :: None) , PathFreshness :: LastModifiedUpstream { upstream : sha }) ; }) ; }
};
}
