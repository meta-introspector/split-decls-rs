// Generated macro for scalar_skip_false (function)
macro_rules! Depcrate_executor_tests_directivesscalar_skip_false {
() => {
// Module: crate::executor_tests::directives
// Provides: {"scalar_skip_false"}
// Dependencies: {}
# [tokio :: test] async fn scalar_skip_false () { run_query ("{ a, b @skip(if: false) }" , | result | { assert_eq ! (result . get_field_value ("a") , Some (& graphql :: value ! ("a"))) ; assert_eq ! (result . get_field_value ("b") , Some (& graphql :: value ! ("b"))) ; }) . await ; }
};
}
