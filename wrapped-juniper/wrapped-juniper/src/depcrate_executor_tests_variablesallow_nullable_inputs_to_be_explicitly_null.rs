// Generated macro for allow_nullable_inputs_to_be_explicitly_null (function)
macro_rules! Depcrate_executor_tests_variablesallow_nullable_inputs_to_be_explicitly_null {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_nullable_inputs_to_be_explicitly_null"}
// Dependencies: {}
# [tokio :: test] async fn allow_nullable_inputs_to_be_explicitly_null () { run_query (r#"{ fieldWithNullableStringInput(input: null) }"# , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithNullableStringInput") , Some (& graphql :: value ! (r#"None"#))) ; } ,) . await ; }
};
}
