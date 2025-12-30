// Generated macro for variable_error_on_incorrect_type (function)
macro_rules! Depcrate_executor_tests_variablesvariable_error_on_incorrect_type {
() => {
// Module: crate::executor_tests::variables
// Provides: {"variable_error_on_incorrect_type"}
// Dependencies: {}
# [tokio :: test] async fn variable_error_on_incorrect_type () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($input: TestInputObject) { fieldWithObjectInput(input: $input) }"# ; let vars = graphql :: vars ! { "input" : "foo bar" } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new (r#"Variable "$input" got invalid value. Expected "TestInputObject", found not an object."# , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; }
};
}
