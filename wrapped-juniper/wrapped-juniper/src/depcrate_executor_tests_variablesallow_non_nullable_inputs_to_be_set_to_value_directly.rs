// Generated macro for allow_non_nullable_inputs_to_be_set_to_value_directly (function)
macro_rules! Depcrate_executor_tests_variablesallow_non_nullable_inputs_to_be_set_to_value_directly {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_non_nullable_inputs_to_be_set_to_value_directly"}
// Dependencies: {}
# [tokio :: test] async fn allow_non_nullable_inputs_to_be_set_to_value_directly () { run_query (r#"{ fieldWithNonNullableStringInput(input: "a") }"# , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithNonNullableStringInput") , Some (& graphql :: value ! (r#""a""#)) ,) ; } ,) . await ; }
};
}
