// Generated macro for test_local_committed_modifications_subdirectory (function)
macro_rules! Depcrate_core_config_teststest_local_committed_modifications_subdirectory {
() => {
// Module: crate::core::config::tests
// Provides: {"test_local_committed_modifications_subdirectory"}
// Dependencies: {}
# [test] fn test_local_committed_modifications_subdirectory () { git_test (| ctx | { let sha = ctx . create_upstream_merge (& ["a/b/c"]) ; ctx . create_upstream_merge (& ["b" , "c"]) ; ctx . create_branch ("feature") ; ctx . modify ("a/b/d") ; ctx . commit () ; assert_eq ! (ctx . check_modifications (& ["a/b"] , CiEnv :: None) , PathFreshness :: HasLocalModifications { upstream : sha }) ; }) ; }
};
}
