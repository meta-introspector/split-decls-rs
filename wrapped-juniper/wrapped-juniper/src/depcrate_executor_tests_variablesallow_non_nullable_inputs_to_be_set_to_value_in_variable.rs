// Generated macro for allow_non_nullable_inputs_to_be_set_to_value_in_variable (function)
macro_rules! Depcrate_executor_tests_variablesallow_non_nullable_inputs_to_be_set_to_value_in_variable {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_non_nullable_inputs_to_be_set_to_value_in_variable"}
// Dependencies: {}
# [tokio :: test] async fn allow_non_nullable_inputs_to_be_set_to_value_in_variable () { run_variable_query (r#"query q($value: String!) { fieldWithNonNullableStringInput(input: $value) }"# , graphql :: vars ! { "value" : "a" } , | result | { assert_eq ! (result . get_field_value ("fieldWithNonNullableStringInput") , Some (& graphql :: value ! (r#""a""#)) ,) ; } ,) . await ; }
};
}
