// Generated macro for does_not_allow_non_null_lists_of_non_null_to_be_null (function)
macro_rules! Depcrate_executor_tests_variablesdoes_not_allow_non_null_lists_of_non_null_to_be_null {
() => {
// Module: crate::executor_tests::variables
// Provides: {"does_not_allow_non_null_lists_of_non_null_to_be_null"}
// Dependencies: {}
# [tokio :: test] async fn does_not_allow_non_null_lists_of_non_null_to_be_null () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($input: [String!]!) { nnListNn(input: $input) }"# ; let vars = graphql :: vars ! { "input" : null } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new (r#"Variable "$input" of required type "[String!]!" was not provided."# , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; }
};
}
