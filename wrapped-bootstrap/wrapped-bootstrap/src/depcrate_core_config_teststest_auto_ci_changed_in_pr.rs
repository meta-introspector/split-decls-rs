// Generated macro for test_auto_ci_changed_in_pr (function)
macro_rules! Depcrate_core_config_teststest_auto_ci_changed_in_pr {
() => {
// Module: crate::core::config::tests
// Provides: {"test_auto_ci_changed_in_pr"}
// Dependencies: {}
# [test] fn test_auto_ci_changed_in_pr () { git_test (| ctx | { let sha = ctx . create_upstream_merge (& ["a"]) ; ctx . create_upstream_merge (& ["b" , "c"]) ; let src = ctx . check_modifications (& ["c" , "d"] , CiEnv :: GitHubActions) ; assert_eq ! (src , PathFreshness :: HasLocalModifications { upstream : sha }) ; }) ; }
};
}
