// Generated macro for variable_runs_from_input_value_on_scalar (function)
macro_rules! Depcrate_executor_tests_variablesvariable_runs_from_input_value_on_scalar {
() => {
// Module: crate::executor_tests::variables
// Provides: {"variable_runs_from_input_value_on_scalar"}
// Dependencies: {}
# [tokio :: test] async fn variable_runs_from_input_value_on_scalar () { run_variable_query (r#"query q($input: TestInputObject) { fieldWithObjectInput(input: $input) }"# , graphql :: vars ! { "input" : { "c" : "baz" , "d" : "SerializedValue" , } , } , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithObjectInput") , Some (& graphql :: value ! (r#"Some(TestInputObject { a: None, b: None, c: "baz", d: Some(TestComplexScalar) })"#))) ; } ,) . await ; }
};
}
