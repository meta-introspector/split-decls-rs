// Generated macro for impl_1075 (impl)
macro_rules! Depcrate_ssl_test_serverimpl_1075 {
() => {
// Module: crate::ssl::test::server
// Provides: {"impl_1075"}
// Dependencies: {}
impl ClientSslBuilder { pub fn ssl (& mut self) -> & mut SslRef { & mut self . ssl } pub fn connect (self) -> SslStream < TcpStream > { let socket = TcpStream :: connect (self . addr) . unwrap () ; let mut s = self . ssl . connect (socket) . unwrap () ; s . read_exact (& mut [0]) . unwrap () ; s } pub fn connect_err (self) { let socket = TcpStream :: connect (self . addr) . unwrap () ; self . ssl . connect (socket) . unwrap_err () ; } }
};
}
