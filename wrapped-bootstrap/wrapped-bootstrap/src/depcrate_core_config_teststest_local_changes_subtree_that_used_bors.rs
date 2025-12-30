// Generated macro for test_local_changes_subtree_that_used_bors (function)
macro_rules! Depcrate_core_config_teststest_local_changes_subtree_that_used_bors {
() => {
// Module: crate::core::config::tests
// Provides: {"test_local_changes_subtree_that_used_bors"}
// Dependencies: {}
# [test] fn test_local_changes_subtree_that_used_bors () { git_test (| ctx | { ctx . create_upstream_merge (& ["a"]) ; ctx . run_git (& ["switch" , "--orphan" , "subtree"]) ; ctx . modify ("bar") ; ctx . commit () ; ctx . create_upstream_merge (& ["subtree/a"]) ; ctx . run_git (& ["commit" , "--amend" , "--date" , "Wed Feb 16 14:00 2011 +0100" , "--no-edit"]) ; ctx . switch_to_branch ("main") ; ctx . run_git (& ["merge" , "subtree" , "--allow-unrelated"]) ; let upstream_1 = ctx . create_upstream_merge (& ["x"]) ; let upstream_2 = ctx . create_upstream_merge (& ["a"]) ; ctx . switch_to_branch ("subtree") ; ctx . create_branch ("subtree-pr") ; ctx . modify ("baz") ; ctx . commit () ; ctx . merge ("main" , "committer <committer@foo.bar>") ; ctx . switch_to_branch ("subtree") ; ctx . merge ("subtree-pr" , "committer <committer@foo.bar>") ; assert_eq ! (ctx . check_modifications (& ["x"] , CiEnv :: None) , PathFreshness :: LastModifiedUpstream { upstream : upstream_1 }) ; assert_eq ! (ctx . check_modifications (& ["nonexistent"] , CiEnv :: None) , PathFreshness :: LastModifiedUpstream { upstream : upstream_2 }) ; }) ; }
};
}
