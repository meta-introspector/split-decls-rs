// Generated macro for allow_non_null_lists_of_non_null_to_contain_values (function)
macro_rules! Depcrate_executor_tests_variablesallow_non_null_lists_of_non_null_to_contain_values {
() => {
// Module: crate::executor_tests::variables
// Provides: {"allow_non_null_lists_of_non_null_to_contain_values"}
// Dependencies: {}
# [tokio :: test] async fn allow_non_null_lists_of_non_null_to_contain_values () { run_variable_query (r#"query q($input: [String!]!) { nnListNn(input: $input) }"# , graphql :: vars ! { "input" : ["A"] } , | result | { assert_eq ! (result . get_field_value ("nnListNn") , Some (& graphql :: value ! (r#"["A"]"#)) ,) ; } ,) . await ; }
};
}
