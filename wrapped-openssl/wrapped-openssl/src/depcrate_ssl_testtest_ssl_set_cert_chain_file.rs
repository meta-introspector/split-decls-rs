// Generated macro for test_ssl_set_cert_chain_file (function)
macro_rules! Depcrate_ssl_testtest_ssl_set_cert_chain_file {
() => {
// Module: crate::ssl::test
// Provides: {"test_ssl_set_cert_chain_file"}
// Dependencies: {}
# [test] # [cfg (ossl110)] fn test_ssl_set_cert_chain_file () { let ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () . build () ; let mut ssl = Ssl :: new (& ctx) . unwrap () ; ssl . set_certificate_chain_file ("test/cert.pem") . unwrap () ; }
};
}
