// Generated macro for test_save_der (function)
macro_rules! Depcrate_x509_teststest_save_der {
() => {
// Module: crate::x509::tests
// Provides: {"test_save_der"}
// Dependencies: {}
# [test] fn test_save_der () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let der = cert . to_der () . unwrap () ; assert ! (! der . is_empty ()) ; }
};
}
