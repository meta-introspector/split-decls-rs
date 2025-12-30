// Generated macro for variable_complex_input (function)
macro_rules! Depcrate_executor_tests_variablesvariable_complex_input {
() => {
// Module: crate::executor_tests::variables
// Provides: {"variable_complex_input"}
// Dependencies: {}
# [tokio :: test] async fn variable_complex_input () { run_variable_query (r#"query q($input: TestInputObject) { fieldWithObjectInput(input: $input) }"# , graphql :: vars ! { "input" : { "a" : "foo" , "b" : ["bar"] , "c" : "baz" , } , } , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithObjectInput") , Some (& graphql :: value ! (r#"Some(TestInputObject { a: Some("foo"), b: Some([Some("bar")]), c: "baz", d: None })"#))) ; } ,) . await ; }
};
}
