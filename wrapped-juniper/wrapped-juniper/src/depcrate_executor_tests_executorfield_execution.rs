// Generated macro for field_execution (module)
macro_rules! Depcrate_executor_tests_executorfield_execution {
() => {
// Module: crate::executor_tests::executor
// Provides: {"field_execution"}
// Dependencies: {}
mod field_execution { use crate :: { graphql , graphql_object , schema :: model :: RootNode , types :: scalars :: { EmptyMutation , EmptySubscription } , } ; struct DataType ; struct DeepDataType ; # [graphql_object] impl DataType { fn a () -> & 'static str { "Apple" } fn b () -> & 'static str { "Banana" } fn c () -> & 'static str { "Cookie" } fn d () -> & 'static str { "Donut" } fn e () -> & 'static str { "Egg" } fn f () -> & 'static str { "Fish" } fn pic (size : Option < i32 >) -> String { format ! ("Pic of size: {}" , size . unwrap_or (50)) } fn deep () -> DeepDataType { DeepDataType } } # [graphql_object] impl DeepDataType { fn a () -> & 'static str { "Already Been Done" } fn b () -> & 'static str { "Boring" } fn c () -> Vec < Option < & 'static str > > { vec ! [Some ("Contrived") , None , Some ("Confusing")] } fn deeper () -> Vec < Option < DataType > > { vec ! [Some (DataType) , None , Some (DataType)] } } # [tokio :: test] async fn test () { let schema = RootNode :: new (DataType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let doc = r"
            query Example($size: Int) {
                a,
                b,
                x: c
                ...c
                f
                ...on DataType {
                    pic(size: $size)
                }
                deep {
                    a
                    b
                    c
                    deeper {
                        a
                        b
                    }
                }
            }

            fragment c on DataType {
                d
                e
            }
        " ; let vars = graphql :: vars ! { "size" : 100 } ; let (result , errs) = crate :: execute (doc , None , & schema , & vars , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; assert_eq ! (result , graphql :: value ! ({ "a" : "Apple" , "b" : "Banana" , "x" : "Cookie" , "d" : "Donut" , "e" : "Egg" , "f" : "Fish" , "pic" : "Pic of size: 100" , "deep" : { "a" : "Already Been Done" , "b" : "Boring" , "c" : ["Contrived" , null , "Confusing"] , "deeper" : [{ "a" : "Apple" , "b" : "Banana" , } , null , { "a" : "Apple" , "b" : "Banana" , } ,] , } , }) ,) ; } }
};
}
