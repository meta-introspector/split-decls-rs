// Generated macro for test_no_completions_in_for_loop_in_kw_pos (function)
macro_rules! Depcrate_context_teststest_no_completions_in_for_loop_in_kw_pos {
() => {
// Module: crate::context::tests
// Provides: {"test_no_completions_in_for_loop_in_kw_pos"}
// Dependencies: {}
# [test] fn test_no_completions_in_for_loop_in_kw_pos () { assert_eq ! (completion_list (r#"fn foo() { for i i$0 }"#) , String :: new ()) ; assert_eq ! (completion_list (r#"fn foo() { for i in$0 }"#) , String :: new ()) ; }
};
}
