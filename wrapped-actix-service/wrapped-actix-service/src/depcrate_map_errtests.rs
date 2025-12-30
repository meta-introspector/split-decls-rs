// Generated macro for tests (module)
macro_rules! Depcrate_map_errtests {
() => {
// Module: crate::map_err
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use futures_util :: future :: lazy ; use super :: * ; use crate :: { err , ok , IntoServiceFactory , Ready , ServiceExt , ServiceFactoryExt } ; struct Srv ; impl Service < () > for Srv { type Response = () ; type Error = () ; type Future = Ready < Result < () , () > > ; fn poll_ready (& self , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Err (())) } fn call (& self , _ : ()) -> Self :: Future { err (()) } } # [actix_rt :: test] async fn test_poll_ready () { let srv = Srv . map_err (| _ | "error") ; let res = lazy (| cx | srv . poll_ready (cx)) . await ; assert_eq ! (res , Poll :: Ready (Err ("error"))) ; } # [actix_rt :: test] async fn test_call () { let srv = Srv . map_err (| _ | "error") ; let res = srv . call (()) . await ; assert ! (res . is_err ()) ; assert_eq ! (res . err () . unwrap () , "error") ; } # [actix_rt :: test] async fn test_new_service () { let new_srv = (| | ok :: < _ , () > (Srv)) . into_factory () . map_err (| _ | "error") ; let srv = new_srv . new_service (& ()) . await . unwrap () ; let res = srv . call (()) . await ; assert ! (res . is_err ()) ; assert_eq ! (res . err () . unwrap () , "error") ; } }
};
}
