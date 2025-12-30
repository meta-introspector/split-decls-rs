// Generated macro for test_pr_ci_unchanged_anywhere (function)
macro_rules! Depcrate_core_config_teststest_pr_ci_unchanged_anywhere {
() => {
// Module: crate::core::config::tests
// Provides: {"test_pr_ci_unchanged_anywhere"}
// Dependencies: {}
# [test] fn test_pr_ci_unchanged_anywhere () { git_test (| ctx | { let sha = ctx . create_upstream_merge (& ["a"]) ; ctx . create_nonupstream_merge (& ["b"]) ; let src = ctx . check_modifications (& ["c"] , CiEnv :: GitHubActions) ; assert_eq ! (src , PathFreshness :: LastModifiedUpstream { upstream : sha }) ; }) ; }
};
}
