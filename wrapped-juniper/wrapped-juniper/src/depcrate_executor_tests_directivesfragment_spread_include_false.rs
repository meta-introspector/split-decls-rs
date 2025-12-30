// Generated macro for fragment_spread_include_false (function)
macro_rules! Depcrate_executor_tests_directivesfragment_spread_include_false {
() => {
// Module: crate::executor_tests::directives
// Provides: {"fragment_spread_include_false"}
// Dependencies: {}
# [tokio :: test] async fn fragment_spread_include_false () { run_query ("{ a, ...Frag @include(if: false) } fragment Frag on TestType { b }" , | result | { assert_eq ! (result . get_field_value ("a") , Some (& graphql :: value ! ("a"))) ; assert_eq ! (result . get_field_value ("b") , None) ; } ,) . await ; }
};
}
