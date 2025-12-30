// Generated macro for tests (module)
macro_rules! Depcrate_test_servertests {
() => {
// Module: crate::test_server
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use actix_service :: fn_service ; use super :: * ; # [tokio :: test] async fn connect_in_tokio_runtime () { let srv = TestServer :: start (| | fn_service (| _sock | async move { Ok :: < _ , () > (()) })) ; assert ! (srv . connect () . is_ok ()) ; } # [actix_rt :: test] async fn connect_in_actix_runtime () { let srv = TestServer :: start (| | fn_service (| _sock | async move { Ok :: < _ , () > (()) })) ; assert ! (srv . connect () . is_ok ()) ; } }
};
}
