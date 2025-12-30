// Generated macro for test_local_changes_negative_path (function)
macro_rules! Depcrate_core_config_teststest_local_changes_negative_path {
() => {
// Module: crate::core::config::tests
// Provides: {"test_local_changes_negative_path"}
// Dependencies: {}
# [test] fn test_local_changes_negative_path () { git_test (| ctx | { let upstream = ctx . create_upstream_merge (& ["a"]) ; ctx . create_branch ("feature") ; ctx . modify ("b") ; ctx . modify ("d") ; ctx . commit () ; assert_eq ! (ctx . check_modifications (& [":!b" , ":!d"] , CiEnv :: None) , PathFreshness :: LastModifiedUpstream { upstream : upstream . clone () }) ; assert_eq ! (ctx . check_modifications (& [":!c"] , CiEnv :: None) , PathFreshness :: HasLocalModifications { upstream : upstream . clone () }) ; assert_eq ! (ctx . check_modifications (& [":!d" , ":!x"] , CiEnv :: None) , PathFreshness :: HasLocalModifications { upstream }) ; }) ; }
};
}
