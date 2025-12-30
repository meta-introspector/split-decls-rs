// Generated macro for test_local_no_upstream_commit_with_changes (function)
macro_rules! Depcrate_core_config_teststest_local_no_upstream_commit_with_changes {
() => {
// Module: crate::core::config::tests
// Provides: {"test_local_no_upstream_commit_with_changes"}
// Dependencies: {}
# [test] fn test_local_no_upstream_commit_with_changes () { git_test (| ctx | { ctx . create_upstream_merge (& ["a" , "e"]) ; ctx . create_upstream_merge (& ["a" , "e"]) ; let sha = ctx . create_upstream_merge (& ["a" , "e"]) ; ctx . create_branch ("feature") ; ctx . modify ("d") ; ctx . commit () ; assert_eq ! (ctx . check_modifications (& ["x"] , CiEnv :: None) , PathFreshness :: LastModifiedUpstream { upstream : sha }) ; }) ; }
};
}
