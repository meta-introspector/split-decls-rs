// Generated macro for impl_1073 (impl)
macro_rules! Depcrate_ssl_test_serverimpl_1073 {
() => {
// Module: crate::ssl::test::server
// Provides: {"impl_1073"}
// Dependencies: {}
impl Client { pub fn builder (& self) -> ClientSslBuilder { ClientSslBuilder { ssl : Ssl :: new (& self . ctx) . unwrap () , addr : self . addr , } } }
};
}
