// Generated macro for cert_store (function)
macro_rules! Depcrate_ssl_testcert_store {
() => {
// Module: crate::ssl::test
// Provides: {"cert_store"}
// Dependencies: {}
# [test] fn cert_store () { let server = Server :: builder () . build () ; let mut client = server . client () ; let cert = X509 :: from_pem (ROOT_CERT) . unwrap () ; client . ctx () . cert_store_mut () . add_cert (cert) . unwrap () ; client . ctx () . set_verify (SslVerifyMode :: PEER) ; client . connect () ; }
};
}
