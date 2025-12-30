// Generated macro for does_not_accept_string_literals (function)
macro_rules! Depcrate_executor_tests_enumsdoes_not_accept_string_literals {
() => {
// Module: crate::executor_tests::enums
// Provides: {"does_not_accept_string_literals"}
// Dependencies: {}
# [tokio :: test] async fn does_not_accept_string_literals () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"{ toString(color: "RED") }"# ; let vars = graphql :: vars ! { } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new (r#"Invalid value for argument "color", reason: Invalid value ""RED"" for enum "Color""# , & [SourcePosition :: new (18 , 0 , 18)] ,) . into () ,) ; }
};
}
