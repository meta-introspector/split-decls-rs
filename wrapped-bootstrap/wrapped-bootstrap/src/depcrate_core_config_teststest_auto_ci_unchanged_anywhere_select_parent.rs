// Generated macro for test_auto_ci_unchanged_anywhere_select_parent (function)
macro_rules! Depcrate_core_config_teststest_auto_ci_unchanged_anywhere_select_parent {
() => {
// Module: crate::core::config::tests
// Provides: {"test_auto_ci_unchanged_anywhere_select_parent"}
// Dependencies: {}
# [test] fn test_auto_ci_unchanged_anywhere_select_parent () { git_test (| ctx | { let sha = ctx . create_upstream_merge (& ["a"]) ; ctx . create_upstream_merge (& ["b"]) ; let src = ctx . check_modifications (& ["c"] , CiEnv :: GitHubActions) ; assert_eq ! (src , PathFreshness :: LastModifiedUpstream { upstream : sha }) ; }) ; }
};
}
