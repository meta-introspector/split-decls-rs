// Generated macro for allow_lists_to_be_null (function)
macro_rules! Depcrate_executor_tests_variablesallow_lists_to_be_null {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_lists_to_be_null"}
// Dependencies: {}
# [tokio :: test] async fn allow_lists_to_be_null () { run_variable_query (r#"query q($input: [String]) { list(input: $input) }"# , graphql :: vars ! { "input" : null } , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("list") , Some (& graphql :: value ! (r#"None"#)) ,) ; } ,) . await ; }
};
}
