// Generated macro for allow_lists_to_contain_values (function)
macro_rules! Depcrate_executor_tests_variablesallow_lists_to_contain_values {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_lists_to_contain_values"}
// Dependencies: {}
# [tokio :: test] async fn allow_lists_to_contain_values () { run_variable_query (r#"query q($input: [String]) { list(input: $input) }"# , graphql :: vars ! { "input" : ["A"] } , | result | { assert_eq ! (result . get_field_value ("list") , Some (& graphql :: value ! (r#"Some([Some("A")])"#)) ,) ; } ,) . await ; }
};
}
