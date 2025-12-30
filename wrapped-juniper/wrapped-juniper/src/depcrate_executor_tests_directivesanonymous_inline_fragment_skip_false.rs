// Generated macro for anonymous_inline_fragment_skip_false (function)
macro_rules! Depcrate_executor_tests_directivesanonymous_inline_fragment_skip_false {
() => {
// Module: crate::executor_tests::directives
// Provides: {"anonymous_inline_fragment_skip_false"}
// Dependencies: {}
# [tokio :: test] async fn anonymous_inline_fragment_skip_false () { run_query ("{ a, ... @skip(if: false) { b } }" , | result | { assert_eq ! (result . get_field_value ("a") , Some (& graphql :: value ! ("a"))) ; assert_eq ! (result . get_field_value ("b") , Some (& graphql :: value ! ("b"))) ; }) . await ; }
};
}
