// Generated macro for fragment_spread_skip_true (function)
macro_rules! Depcrate_executor_tests_directivesfragment_spread_skip_true {
() => {
// Module: crate::executor_tests::directives
// Provides: {"fragment_spread_skip_true"}
// Dependencies: {}
# [tokio :: test] async fn fragment_spread_skip_true () { run_query ("{ a, ...Frag @skip(if: true) } fragment Frag on TestType { b }" , | result | { assert_eq ! (result . get_field_value ("a") , Some (& graphql :: value ! ("a"))) ; assert_eq ! (result . get_field_value ("b") , None) ; } ,) . await ; }
};
}
