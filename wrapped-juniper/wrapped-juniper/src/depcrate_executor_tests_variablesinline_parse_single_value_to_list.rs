// Generated macro for inline_parse_single_value_to_list (function)
macro_rules! Depcrate_executor_tests_variablesinline_parse_single_value_to_list {
() => {
// Module: crate::executor_tests::variables
// Provides: {"inline_parse_single_value_to_list"}
// Dependencies: {}
# [tokio :: test] async fn inline_parse_single_value_to_list () { run_query (r#"{ fieldWithObjectInput(input: {a: "foo", b: "bar", c: "baz"}) }"# , | result : & Object < DefaultScalarValue > | { assert_eq ! (result . get_field_value ("fieldWithObjectInput") , Some (& graphql :: value ! (r#"Some(TestInputObject { a: Some("foo"), b: Some([Some("bar")]), c: "baz", d: None })"#))) ; } ,) . await ; }
};
}
