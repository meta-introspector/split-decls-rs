// Generated macro for input_object_with_default_values (function)
macro_rules! Depcrate_executor_tests_variablesinput_object_with_default_values {
() => {
// Module: crate::executor_tests::variables
// Provides: {"input_object_with_default_values"}
// Dependencies: {}
# [tokio :: test] async fn input_object_with_default_values () { run_query (r#"{ inputWithDefaults(arg: {a: 1}) }"# , | result | { assert_eq ! (result . get_field_value ("inputWithDefaults") , Some (& graphql :: value ! (r#"a: 1"#)) ,) ; }) . await ; run_variable_query (r#"query q($var: Int!) { inputWithDefaults(arg: {a: $var}) }"# , graphql :: vars ! { "var" : 1 } , | result | { assert_eq ! (result . get_field_value ("inputWithDefaults") , Some (& graphql :: value ! (r#"a: 1"#)) ,) ; } ,) . await ; run_variable_query (r#"query q($var: Int = 1) { inputWithDefaults(arg: {a: $var}) }"# , graphql :: vars ! { } , | result | { assert_eq ! (result . get_field_value ("inputWithDefaults") , Some (& graphql :: value ! (r#"a: 1"#)) ,) ; } ,) . await ; run_variable_query (r#"query q($var: Int = 1) { inputWithDefaults(arg: {a: $var}) }"# , graphql :: vars ! { "var" : 2 } , | result | { assert_eq ! (result . get_field_value ("inputWithDefaults") , Some (& graphql :: value ! (r#"a: 2"#)) ,) ; } ,) . await ; }
};
}
