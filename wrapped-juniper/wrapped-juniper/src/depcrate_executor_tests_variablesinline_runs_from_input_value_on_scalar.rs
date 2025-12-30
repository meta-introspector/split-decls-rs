// Generated macro for inline_runs_from_input_value_on_scalar (function)
macro_rules! Depcrate_executor_tests_variablesinline_runs_from_input_value_on_scalar {
() => {
// Module: crate::executor_tests::variables
// Provides: {"inline_runs_from_input_value_on_scalar"}
// Dependencies: {}
# [tokio :: test] async fn inline_runs_from_input_value_on_scalar () { run_query (r#"{ fieldWithObjectInput(input: {c: "baz", d: "SerializedValue"}) }"# , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithObjectInput") , Some (& graphql :: value ! (r#"Some(TestInputObject { a: None, b: None, c: "baz", d: Some(TestComplexScalar) })"#))) ; } ,) . await ; }
};
}
