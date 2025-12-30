// Generated macro for merge_parallel_fragments (module)
macro_rules! Depcrate_executor_tests_executormerge_parallel_fragments {
() => {
// Module: crate::executor_tests::executor
// Provides: {"merge_parallel_fragments"}
// Dependencies: {}
mod merge_parallel_fragments { use crate :: { graphql , graphql_object , schema :: model :: RootNode , types :: scalars :: { EmptyMutation , EmptySubscription } , } ; struct Type ; # [graphql_object] impl Type { fn a () -> & 'static str { "Apple" } fn b () -> & 'static str { "Banana" } fn c () -> & 'static str { "Cherry" } fn deep () -> Type { Type } } # [tokio :: test] async fn test () { let schema = RootNode :: new (Type , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let doc = r"
            { a, ...FragOne, ...FragTwo }
            fragment FragOne on Type {
                b
                deep { b, deeper: deep { b } }
            }
            fragment FragTwo on Type {
                c
                deep { c, deeper: deep { c } }
            }
        " ; let vars = graphql :: vars ! { } ; let (result , errs) = crate :: execute (doc , None , & schema , & vars , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; assert_eq ! (result , graphql :: value ! ({ "a" : "Apple" , "b" : "Banana" , "deep" : { "b" : "Banana" , "deeper" : { "b" : "Banana" , "c" : "Cherry" , } , "c" : "Cherry" , } , "c" : "Cherry" , }) ,) ; } }
};
}
