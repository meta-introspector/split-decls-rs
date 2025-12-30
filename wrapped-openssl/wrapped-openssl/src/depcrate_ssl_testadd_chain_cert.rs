// Generated macro for add_chain_cert (function)
macro_rules! Depcrate_ssl_testadd_chain_cert {
() => {
// Module: crate::ssl::test
// Provides: {"add_chain_cert"}
// Dependencies: {}
# [test] # [cfg (ossl102)] fn add_chain_cert () { let ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () . build () ; let cert = X509 :: from_pem (CERT) . unwrap () ; let mut ssl = Ssl :: new (& ctx) . unwrap () ; assert ! (ssl . add_chain_cert (cert) . is_ok ()) ; }
};
}
