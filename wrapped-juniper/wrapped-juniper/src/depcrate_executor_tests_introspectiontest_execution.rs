// Generated macro for test_execution (function)
macro_rules! Depcrate_executor_tests_introspectiontest_execution {
() => {
// Module: crate::executor_tests::introspection
// Provides: {"test_execution"}
// Dependencies: {}
# [tokio :: test] async fn test_execution () { let doc = r#"
    {
        sampleEnum
        first: sampleScalar(first: 0)
        second: sampleScalar(first: 10 second: 20)
    }
    "# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let (result , errs) = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; assert_eq ! (result , graphql :: value ! ({ "sampleEnum" : "ONE" , "first" : 123 , "second" : 30 , }) ,) ; }
};
}
