// Generated macro for TestServerHandle (struct)
macro_rules! Depcrate_test_serverTestServerHandle {
() => {
// Module: crate::test_server
// Provides: {"TestServerHandle"}
// Dependencies: {}
# [doc = " Test server handle."] pub struct TestServerHandle { addr : net :: SocketAddr , host : String , port : u16 , server_handle : ServerHandle , thread_handle : Option < thread :: JoinHandle < io :: Result < () > > > , }
};
}
