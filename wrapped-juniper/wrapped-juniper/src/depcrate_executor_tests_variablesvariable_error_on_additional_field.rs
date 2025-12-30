// Generated macro for variable_error_on_additional_field (function)
macro_rules! Depcrate_executor_tests_variablesvariable_error_on_additional_field {
() => {
// Module: crate::executor_tests::variables
// Provides: {"variable_error_on_additional_field"}
// Dependencies: {}
# [tokio :: test] async fn variable_error_on_additional_field () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($input: TestInputObject) { fieldWithObjectInput(input: $input) }"# ; let vars = graphql :: vars ! { "input" : { "a" : "foo" , "b" : "bar" , "c" : "baz" , "extra" : "dog" , } , } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new (r#"Variable "$input" got invalid value. In field "extra": Unknown field."# , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; }
};
}
