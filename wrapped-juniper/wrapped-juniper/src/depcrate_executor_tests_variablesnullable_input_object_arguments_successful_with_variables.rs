// Generated macro for nullable_input_object_arguments_successful_with_variables (function)
macro_rules! Depcrate_executor_tests_variablesnullable_input_object_arguments_successful_with_variables {
() => {
// Module: crate::executor_tests::variables
// Provides: {"nullable_input_object_arguments_successful_with_variables"}
// Dependencies: {}
# [tokio :: test] async fn nullable_input_object_arguments_successful_with_variables () { run_variable_query (r#"query q($var: Int!) { exampleInput(arg: {b: $var}) }"# , graphql :: vars ! { "var" : 123 } , | result | { assert_eq ! (result . get_field_value ("exampleInput") , Some (& graphql :: value ! (r#"a: None, b: 123"#)) ,) ; } ,) . await ; run_variable_query (r#"query q($var: String) { exampleInput(arg: {a: $var, b: 1}) }"# , graphql :: vars ! { "var" : null } , | result | { assert_eq ! (result . get_field_value ("exampleInput") , Some (& graphql :: value ! (r#"a: None, b: 1"#)) ,) ; } ,) . await ; run_variable_query (r#"query q($var: String) { exampleInput(arg: {a: $var, b: 1}) }"# , graphql :: vars ! { } , | result | { assert_eq ! (result . get_field_value ("exampleInput") , Some (& graphql :: value ! (r#"a: None, b: 1"#)) ,) ; } ,) . await ; }
};
}
