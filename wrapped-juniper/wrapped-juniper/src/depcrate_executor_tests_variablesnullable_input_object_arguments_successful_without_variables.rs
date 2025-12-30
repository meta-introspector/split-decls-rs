// Generated macro for nullable_input_object_arguments_successful_without_variables (function)
macro_rules! Depcrate_executor_tests_variablesnullable_input_object_arguments_successful_without_variables {
() => {
// Module: crate::executor_tests::variables
// Provides: {"nullable_input_object_arguments_successful_without_variables"}
// Dependencies: {}
# [tokio :: test] async fn nullable_input_object_arguments_successful_without_variables () { run_query (r#"{ exampleInput(arg: {a: "abc", b: 123}) }"# , | result | { assert_eq ! (result . get_field_value ("exampleInput") , Some (& graphql :: value ! (r#"a: Some("abc"), b: 123"#)) ,) ; }) . await ; run_query (r#"{ exampleInput(arg: {a: null, b: 1}) }"# , | result | { assert_eq ! (result . get_field_value ("exampleInput") , Some (& graphql :: value ! (r#"a: None, b: 1"#)) ,) ; }) . await ; }
};
}
