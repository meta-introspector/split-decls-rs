// Generated macro for add_extra_chain_cert (function)
macro_rules! Depcrate_ssl_testadd_extra_chain_cert {
() => {
// Module: crate::ssl::test
// Provides: {"add_extra_chain_cert"}
// Dependencies: {}
# [test] fn add_extra_chain_cert () { let cert = X509 :: from_pem (CERT) . unwrap () ; let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . add_extra_chain_cert (cert) . unwrap () ; }
};
}
