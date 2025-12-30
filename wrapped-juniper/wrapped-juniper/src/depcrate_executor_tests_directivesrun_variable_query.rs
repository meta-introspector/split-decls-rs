// Generated macro for run_variable_query (function)
macro_rules! Depcrate_executor_tests_directivesrun_variable_query {
() => {
// Module: crate::executor_tests::directives
// Provides: {"run_variable_query"}
// Dependencies: {}
async fn run_variable_query < F > (query : & str , vars : Variables < DefaultScalarValue > , f : F) where F : Fn (& Object < DefaultScalarValue >) , { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let (result , errs) = crate :: execute (query , None , & schema , & vars , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; let obj = result . as_object_value () . expect ("Result is not an object") ; f (obj) ; }
};
}
