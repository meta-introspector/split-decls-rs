// Generated macro for default_argument_when_nullable_variable_not_provided (function)
macro_rules! Depcrate_executor_tests_variablesdefault_argument_when_nullable_variable_not_provided {
() => {
// Module: crate::executor_tests::variables
// Provides: {"default_argument_when_nullable_variable_not_provided"}
// Dependencies: {}
# [tokio :: test] async fn default_argument_when_nullable_variable_not_provided () { run_query (r#"query q($input: String) { nullableFieldWithDefaultArgumentValue(input: $input) }"# , | result | { assert_eq ! (result . get_field_value ("nullableFieldWithDefaultArgumentValue") , Some (& graphql :: value ! (r#"Some("Hello World")"#)) ,) ; } ,) . await ; }
};
}
