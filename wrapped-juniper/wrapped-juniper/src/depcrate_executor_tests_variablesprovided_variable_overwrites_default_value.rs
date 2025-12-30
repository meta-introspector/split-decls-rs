// Generated macro for provided_variable_overwrites_default_value (function)
macro_rules! Depcrate_executor_tests_variablesprovided_variable_overwrites_default_value {
() => {
// Module: crate::executor_tests::variables
// Provides: {"provided_variable_overwrites_default_value"}
// Dependencies: {}
# [tokio :: test] async fn provided_variable_overwrites_default_value () { run_variable_query (r#"query q($input: String!) { fieldWithDefaultArgumentValue(input: $input) }"# , graphql :: vars ! { "input" : "Overwritten" } , | result | { assert_eq ! (result . get_field_value ("fieldWithDefaultArgumentValue") , Some (& graphql :: value ! (r#""Overwritten""#)) ,) ; } ,) . await ; }
};
}
