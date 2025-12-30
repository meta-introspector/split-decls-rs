// Generated macro for TestServer (struct)
macro_rules! Depcrate_test_serverTestServer {
() => {
// Module: crate::test_server
// Provides: {"TestServer"}
// Dependencies: {}
# [doc = " A testing server."] # [doc = ""] # [doc = " `TestServer` is very simple test server that simplify process of writing integration tests for"] # [doc = " network applications."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use actix_service::fn_service;"] # [doc = " use actix_server::TestServer;"] # [doc = ""] # [doc = " #[actix_rt::main]"] # [doc = " async fn main() {"] # [doc = "     let srv = TestServer::start(|| fn_service("] # [doc = "         |sock| async move {"] # [doc = "             println!(\"New connection: {:?}\", sock);"] # [doc = "             Ok::<_, ()>(())"] # [doc = "         }"] # [doc = "     ));"] # [doc = ""] # [doc = "     println!(\"SOCKET: {:?}\", srv.connect());"] # [doc = " }"] # [doc = " ```"] pub struct TestServer ;
};
}
