// Generated macro for test_store_all_certificates (function)
macro_rules! Depcrate_x509_teststest_store_all_certificates {
() => {
// Module: crate::x509::tests
// Provides: {"test_store_all_certificates"}
// Dependencies: {}
# [test] # [cfg (ossl300)] fn test_store_all_certificates () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let store = { let mut b = X509StoreBuilder :: new () . unwrap () ; b . add_cert (cert) . unwrap () ; b . build () } ; assert_eq ! (store . all_certificates () . len () , 1) ; }
};
}
