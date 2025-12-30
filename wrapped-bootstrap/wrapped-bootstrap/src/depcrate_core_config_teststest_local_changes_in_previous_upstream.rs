// Generated macro for test_local_changes_in_previous_upstream (function)
macro_rules! Depcrate_core_config_teststest_local_changes_in_previous_upstream {
() => {
// Module: crate::core::config::tests
// Provides: {"test_local_changes_in_previous_upstream"}
// Dependencies: {}
# [test] fn test_local_changes_in_previous_upstream () { git_test (| ctx | { let sha = ctx . create_upstream_merge (& ["a" , "e"]) ; ctx . create_upstream_merge (& ["b" , "c"]) ; ctx . create_branch ("feature") ; ctx . modify ("d") ; ctx . commit () ; assert_eq ! (ctx . check_modifications (& ["a"] , CiEnv :: None) , PathFreshness :: LastModifiedUpstream { upstream : sha }) ; }) ; }
};
}
