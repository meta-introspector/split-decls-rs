// Generated macro for tests (module)
macro_rules! Depcrate_maptests {
() => {
// Module: crate::map
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use futures_util :: future :: lazy ; use super :: * ; use crate :: { ok , IntoServiceFactory , Ready , ServiceExt , ServiceFactoryExt } ; struct Srv ; impl Service < () > for Srv { type Response = () ; type Error = () ; type Future = Ready < Result < () , () > > ; crate :: always_ready ! () ; fn call (& self , _ : ()) -> Self :: Future { ok (()) } } # [actix_rt :: test] async fn test_poll_ready () { let srv = Srv . map (| _ | "ok") ; let res = lazy (| cx | srv . poll_ready (cx)) . await ; assert_eq ! (res , Poll :: Ready (Ok (()))) ; } # [actix_rt :: test] async fn test_call () { let srv = Srv . map (| _ | "ok") ; let res = srv . call (()) . await ; assert ! (res . is_ok ()) ; assert_eq ! (res . unwrap () , "ok") ; } # [actix_rt :: test] async fn test_new_service () { let new_srv = (| | ok :: < _ , () > (Srv)) . into_factory () . map (| _ | "ok") ; let srv = new_srv . new_service (& ()) . await . unwrap () ; let res = srv . call (()) . await ; assert ! (res . is_ok ()) ; assert_eq ! (res . unwrap () , ("ok")) ; } }
};
}
