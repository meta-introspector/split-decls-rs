// Generated macro for verify_trusted_with_set_cert (function)
macro_rules! Depcrate_ssl_testverify_trusted_with_set_cert {
() => {
// Module: crate::ssl::test
// Provides: {"verify_trusted_with_set_cert"}
// Dependencies: {}
# [test] # [cfg (ossl102)] fn verify_trusted_with_set_cert () { let server = Server :: builder () . build () ; let mut store = X509StoreBuilder :: new () . unwrap () ; let x509 = X509 :: from_pem (ROOT_CERT) . unwrap () ; store . add_cert (x509) . unwrap () ; let mut client = server . client () ; client . ctx () . set_verify (SslVerifyMode :: PEER) ; client . ctx () . set_verify_cert_store (store . build ()) . unwrap () ; client . connect () ; }
};
}
