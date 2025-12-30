// Generated macro for default_argument_when_not_provided (function)
macro_rules! Depcrate_executor_tests_variablesdefault_argument_when_not_provided {
() => {
// Module: crate::executor_tests::variables
// Provides: {"default_argument_when_not_provided"}
// Dependencies: {}
# [tokio :: test] async fn default_argument_when_not_provided () { run_query (r#"{ fieldWithDefaultArgumentValue }"# , | result | { assert_eq ! (result . get_field_value ("fieldWithDefaultArgumentValue") , Some (& graphql :: value ! (r#""Hello World""#)) ,) ; }) . await ; }
};
}
