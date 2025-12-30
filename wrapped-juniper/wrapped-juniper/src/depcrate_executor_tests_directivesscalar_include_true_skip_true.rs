// Generated macro for scalar_include_true_skip_true (function)
macro_rules! Depcrate_executor_tests_directivesscalar_include_true_skip_true {
() => {
// Module: crate::executor_tests::directives
// Provides: {"scalar_include_true_skip_true"}
// Dependencies: {}
# [tokio :: test] async fn scalar_include_true_skip_true () { run_query ("{ a, b @include(if: true) @skip(if: true) }" , | result | { assert_eq ! (result . get_field_value ("a") , Some (& graphql :: value ! ("a"))) ; assert_eq ! (result . get_field_value ("b") , None) ; }) . await ; }
};
}
