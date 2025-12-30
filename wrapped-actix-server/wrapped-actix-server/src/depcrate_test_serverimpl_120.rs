// Generated macro for impl_120 (impl)
macro_rules! Depcrate_test_serverimpl_120 {
() => {
// Module: crate::test_server
// Provides: {"impl_120"}
// Dependencies: {}
impl TestServerHandle { # [doc = " Test server host."] pub fn host (& self) -> & str { & self . host } # [doc = " Test server port."] pub fn port (& self) -> u16 { self . port } # [doc = " Get test server address."] pub fn addr (& self) -> net :: SocketAddr { self . addr } # [doc = " Stop server."] fn stop (& mut self) { drop (self . server_handle . stop (false)) ; self . thread_handle . take () . unwrap () . join () . unwrap () . unwrap () ; } # [doc = " Connect to server, returning a Tokio `TcpStream`."] pub fn connect (& self) -> io :: Result < TcpStream > { let stream = net :: TcpStream :: connect (self . addr) ? ; stream . set_nonblocking (true) ? ; TcpStream :: from_std (stream) } }
};
}
