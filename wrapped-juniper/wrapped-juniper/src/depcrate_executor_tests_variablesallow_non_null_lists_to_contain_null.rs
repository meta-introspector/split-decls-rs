// Generated macro for allow_non_null_lists_to_contain_null (function)
macro_rules! Depcrate_executor_tests_variablesallow_non_null_lists_to_contain_null {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_non_null_lists_to_contain_null"}
// Dependencies: {}
# [tokio :: test] async fn allow_non_null_lists_to_contain_null () { run_variable_query (r#"query q($input: [String]!) { nnList(input: $input) }"# , graphql :: vars ! { "input" : ["A" , null , "B"] } , | result | { assert_eq ! (result . get_field_value ("nnList") , Some (& graphql :: value ! (r#"[Some("A"), None, Some("B")]"#)) ,) ; } ,) . await ; }
};
}
