// Generated macro for inline_complex_input (function)
macro_rules! Depcrate_executor_tests_variablesinline_complex_input {
() => {
// Module: crate::executor_tests::variables
// Provides: {"inline_complex_input"}
// Dependencies: {}
# [tokio :: test] async fn inline_complex_input () { run_query (r#"{ fieldWithObjectInput(input: {a: "foo", b: ["bar"], c: "baz"}) }"# , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithObjectInput") , Some (& graphql :: value ! (r#"Some(TestInputObject { a: Some("foo"), b: Some([Some("bar")]), c: "baz", d: None })"#))) ; } ,) . await ; }
};
}
