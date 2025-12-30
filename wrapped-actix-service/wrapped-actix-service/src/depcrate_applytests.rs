// Generated macro for tests (module)
macro_rules! Depcrate_applytests {
() => {
// Module: crate::apply
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use futures_util :: future :: lazy ; use super :: * ; use crate :: { ok , pipeline :: { pipeline , pipeline_factory } , Ready , } ; # [derive (Clone)] struct Srv ; impl Service < () > for Srv { type Response = () ; type Error = () ; type Future = Ready < Result < () , () > > ; crate :: always_ready ! () ; fn call (& self , _ : ()) -> Self :: Future { ok (()) } } # [actix_rt :: test] async fn test_call () { let srv = pipeline (apply_fn (Srv , | req : & 'static str , srv | { let fut = srv . call (()) ; async move { fut . await . unwrap () ; Ok ((req , ())) } })) ; assert_eq ! (lazy (| cx | srv . poll_ready (cx)) . await , Poll :: Ready (Ok (()))) ; let res = srv . call ("srv") . await ; assert ! (res . is_ok ()) ; assert_eq ! (res . unwrap () , ("srv" , ())) ; } # [actix_rt :: test] async fn test_new_service () { let new_srv = pipeline_factory (apply_fn_factory (| | ok :: < _ , () > (Srv) , | req : & 'static str , srv | { let fut = srv . call (()) ; async move { fut . await . unwrap () ; Ok ((req , ())) } } ,)) ; let srv = new_srv . new_service (()) . await . unwrap () ; assert_eq ! (lazy (| cx | srv . poll_ready (cx)) . await , Poll :: Ready (Ok (()))) ; let res = srv . call ("srv") . await ; assert ! (res . is_ok ()) ; assert_eq ! (res . unwrap () , ("srv" , ())) ; } }
};
}
