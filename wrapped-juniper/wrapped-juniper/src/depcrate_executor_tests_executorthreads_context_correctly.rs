// Generated macro for threads_context_correctly (module)
macro_rules! Depcrate_executor_tests_executorthreads_context_correctly {
() => {
// Module: crate::executor_tests::executor
// Provides: {"threads_context_correctly"}
// Dependencies: {}
mod threads_context_correctly { use crate :: { executor :: Context , graphql , graphql_object , schema :: model :: RootNode , types :: scalars :: { EmptyMutation , EmptySubscription } , } ; struct Schema ; struct TestContext { value : String , } impl Context for TestContext { } # [graphql_object (context = TestContext)] impl Schema { fn a (context : & TestContext) -> String { context . value . clone () } } # [tokio :: test] async fn test () { let schema = RootNode :: new (Schema , EmptyMutation :: < TestContext > :: new () , EmptySubscription :: < TestContext > :: new () ,) ; let doc = r"{ a }" ; let vars = graphql :: vars ! { } ; let (result , errs) = crate :: execute (doc , None , & schema , & vars , & TestContext { value : "Context value" . into () , } ,) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; assert_eq ! (result , graphql :: value ! ({ "a" : "Context value" })) ; } }
};
}
