// Generated macro for variable_multiple_errors_with_nesting (function)
macro_rules! Depcrate_executor_tests_variablesvariable_multiple_errors_with_nesting {
() => {
// Module: crate::executor_tests::variables
// Provides: {"variable_multiple_errors_with_nesting"}
// Dependencies: {}
# [tokio :: test] async fn variable_multiple_errors_with_nesting () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($input: TestNestedInputObject) { fieldWithNestedObjectInput(input: $input) }"# ; let vars = graphql :: vars ! { "input" : { "na" : { "a" : "foo" } , } , } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , vec ! [RuleError :: new (r#"Variable "$input" got invalid value. In field "na": In field "c": Expected "String!", found null."# , & [SourcePosition :: new (8 , 0 , 8)] ,) , RuleError :: new (r#"Variable "$input" got invalid value. In field "nb": Expected "String!", found null."# , & [SourcePosition :: new (8 , 0 , 8)] ,) ,] . into () ,) ; }
};
}
