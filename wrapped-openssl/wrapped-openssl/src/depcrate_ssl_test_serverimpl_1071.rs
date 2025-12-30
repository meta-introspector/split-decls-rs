// Generated macro for impl_1071 (impl)
macro_rules! Depcrate_ssl_test_serverimpl_1071 {
() => {
// Module: crate::ssl::test::server
// Provides: {"impl_1071"}
// Dependencies: {}
impl ClientBuilder { pub fn ctx (& mut self) -> & mut SslContextBuilder { & mut self . ctx } pub fn build (self) -> Client { Client { ctx : self . ctx . build () , addr : self . addr , } } pub fn connect (self) -> SslStream < TcpStream > { self . build () . builder () . connect () } pub fn connect_err (self) { self . build () . builder () . connect_err () ; } }
};
}
