// Generated macro for allow_nullable_inputs_to_be_set_to_null_in_variable (function)
macro_rules! Depcrate_executor_tests_variablesallow_nullable_inputs_to_be_set_to_null_in_variable {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_nullable_inputs_to_be_set_to_null_in_variable"}
// Dependencies: {}
# [tokio :: test] async fn allow_nullable_inputs_to_be_set_to_null_in_variable () { run_variable_query (r#"query q($value: String) { fieldWithNullableStringInput(input: $value) }"# , graphql :: vars ! { "value" : null } , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithNullableStringInput") , Some (& graphql :: value ! (r#"None"#))) ; } ,) . await ; }
};
}
