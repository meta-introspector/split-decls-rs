// Generated macro for variable_error_on_omit_non_null (function)
macro_rules! Depcrate_executor_tests_variablesvariable_error_on_omit_non_null {
() => {
// Module: crate::executor_tests::variables
// Provides: {"variable_error_on_omit_non_null"}
// Dependencies: {}
# [tokio :: test] async fn variable_error_on_omit_non_null () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($input: TestInputObject) { fieldWithObjectInput(input: $input) }"# ; let vars = graphql :: vars ! { "input" : { "a" : "foo" , "b" : "bar" , } , } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new (r#"Variable "$input" got invalid value. In field "c": Expected "String!", found null."# , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; }
};
}
