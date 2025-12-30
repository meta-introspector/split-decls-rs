// Generated macro for inline_fragment_include_true (function)
macro_rules! Depcrate_executor_tests_directivesinline_fragment_include_true {
() => {
// Module: crate::executor_tests::directives
// Provides: {"inline_fragment_include_true"}
// Dependencies: {}
# [tokio :: test] async fn inline_fragment_include_true () { run_query ("{ a, ... on TestType @include(if: true) { b } }" , | result | { assert_eq ! (result . get_field_value ("a") , Some (& graphql :: value ! ("a"))) ; assert_eq ! (result . get_field_value ("b") , Some (& graphql :: value ! ("b"))) ; } ,) . await ; }
};
}
