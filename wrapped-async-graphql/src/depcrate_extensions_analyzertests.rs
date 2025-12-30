// Generated macro for tests (module)
macro_rules! Depcrate_extensions_analyzertests {
() => {
// Module: crate::extensions::analyzer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: * ; struct Query ; # [derive (Copy , Clone)] struct MyObj ; # [Object (internal)] impl MyObj { async fn value (& self) -> i32 { 1 } async fn obj (& self) -> MyObj { MyObj } } # [Object (internal)] impl Query { async fn value (& self) -> i32 { 1 } async fn obj (& self) -> MyObj { MyObj } # [graphql (complexity = "count * child_complexity")] async fn objs (& self , count : usize) -> Vec < MyObj > { vec ! [MyObj ; count] } } # [tokio :: test] async fn analyzer () { let schema = Schema :: build (Query , EmptyMutation , EmptySubscription) . extension (extensions :: Analyzer) . finish () ; let res = schema . execute (r#"{
            value obj {
                value obj {
                    value
                }
            }
            objs(count: 10) { value }
        }"# ,) . await . into_result () . unwrap () . extensions . remove ("analyzer") ; assert_eq ! (res , Some (value ! ({ "complexity" : 5 + 10 , "depth" : 3 , }))) ; } }
};
}
