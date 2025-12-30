// Generated macro for allow_nullable_inputs_to_be_omitted (function)
macro_rules! Depcrate_executor_tests_variablesallow_nullable_inputs_to_be_omitted {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_nullable_inputs_to_be_omitted"}
// Dependencies: {}
# [tokio :: test] async fn allow_nullable_inputs_to_be_omitted () { run_query (r#"{ fieldWithNullableStringInput }"# , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithNullableStringInput") , Some (& graphql :: value ! (r#"None"#))) ; } ,) . await ; }
};
}
