// Generated macro for does_not_allow_non_nullable_input_to_be_omitted_in_variable (function)
macro_rules! Depcrate_executor_tests_variablesdoes_not_allow_non_nullable_input_to_be_omitted_in_variable {
() => {
// Module: crate::executor_tests::variables
// Provides: {"does_not_allow_non_nullable_input_to_be_omitted_in_variable"}
// Dependencies: {}
# [tokio :: test] async fn does_not_allow_non_nullable_input_to_be_omitted_in_variable () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($value: String!) { fieldWithNonNullableStringInput(input: $value) }"# ; let vars = graphql :: vars ! { } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new (r#"Variable "$value" of required type "String!" was not provided."# , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; }
};
}
