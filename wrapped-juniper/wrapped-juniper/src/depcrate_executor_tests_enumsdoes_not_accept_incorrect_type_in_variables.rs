// Generated macro for does_not_accept_incorrect_type_in_variables (function)
macro_rules! Depcrate_executor_tests_enumsdoes_not_accept_incorrect_type_in_variables {
() => {
// Module: crate::executor_tests::enums
// Provides: {"does_not_accept_incorrect_type_in_variables"}
// Dependencies: {}
# [tokio :: test] async fn does_not_accept_incorrect_type_in_variables () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($color: Color!) { toString(color: $color) }"# ; let vars = graphql :: vars ! { "color" : 123 } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new (r#"Variable "$color" got invalid value. Expected "Color", found not a string or enum."# , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; }
};
}
