// Generated macro for test_pr_ci_changed_in_pr (function)
macro_rules! Depcrate_core_config_teststest_pr_ci_changed_in_pr {
() => {
// Module: crate::core::config::tests
// Provides: {"test_pr_ci_changed_in_pr"}
// Dependencies: {}
# [test] fn test_pr_ci_changed_in_pr () { git_test (| ctx | { let sha = ctx . create_upstream_merge (& ["a"]) ; ctx . create_nonupstream_merge (& ["b"]) ; let src = ctx . check_modifications (& ["b"] , CiEnv :: GitHubActions) ; assert_eq ! (src , PathFreshness :: HasLocalModifications { upstream : sha }) ; }) ; }
};
}
