// Generated macro for test_local_uncommitted_modifications (function)
macro_rules! Depcrate_core_config_teststest_local_uncommitted_modifications {
() => {
// Module: crate::core::config::tests
// Provides: {"test_local_uncommitted_modifications"}
// Dependencies: {}
# [test] fn test_local_uncommitted_modifications () { git_test (| ctx | { let sha = ctx . create_upstream_merge (& ["a"]) ; ctx . create_branch ("feature") ; ctx . modify ("a") ; assert_eq ! (ctx . check_modifications (& ["a" , "d"] , CiEnv :: None) , PathFreshness :: HasLocalModifications { upstream : sha }) ; }) ; }
};
}
