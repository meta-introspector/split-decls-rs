// Generated macro for does_not_allow_non_null_lists_of_non_null_to_contain_null (function)
macro_rules! Depcrate_executor_tests_variablesdoes_not_allow_non_null_lists_of_non_null_to_contain_null {
() => {
// Module: crate::executor_tests::variables
// Provides: {"does_not_allow_non_null_lists_of_non_null_to_contain_null"}
// Dependencies: {}
# [tokio :: test] async fn does_not_allow_non_null_lists_of_non_null_to_contain_null () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($input: [String!]!) { nnListNn(input: $input) }"# ; let vars = graphql :: vars ! { "input" : ["A" , null , "B"] } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new (r#"Variable "$input" got invalid value. In element #1: Expected "String!", found null."# , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; }
};
}
