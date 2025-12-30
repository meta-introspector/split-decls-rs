// Generated macro for does_not_allow_missing_required_field (function)
macro_rules! Depcrate_executor_tests_variablesdoes_not_allow_missing_required_field {
() => {
// Module: crate::executor_tests::variables
// Provides: {"does_not_allow_missing_required_field"}
// Dependencies: {}
# [tokio :: test] async fn does_not_allow_missing_required_field () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"{ exampleInput(arg: {a: "abc"}) }"# ; let vars = graphql :: vars ! { } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new ("Invalid value for argument \"arg\", \
             reason: \"ExampleInputObject\" is missing fields: \"b\"" , & [SourcePosition :: new (20 , 0 , 20)] ,) . into () ,) ; }
};
}
