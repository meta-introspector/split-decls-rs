// Generated macro for null_when_nullable_variable_of_argument_with_default_value_set_to_null (function)
macro_rules! Depcrate_executor_tests_variablesnull_when_nullable_variable_of_argument_with_default_value_set_to_null {
() => {
// Module: crate::executor_tests::variables
// Provides: {"null_when_nullable_variable_of_argument_with_default_value_set_to_null"}
// Dependencies: {}
# [tokio :: test] async fn null_when_nullable_variable_of_argument_with_default_value_set_to_null () { run_variable_query (r#"query q($input: String) { nullableFieldWithDefaultArgumentValue(input: $input) }"# , graphql :: vars ! { "input" : null } , | result | { assert_eq ! (result . get_field_value ("nullableFieldWithDefaultArgumentValue") , Some (& graphql :: value ! (r#"None"#)) ,) ; } ,) . await ; }
};
}
