// Generated macro for test_save_subject_der (function)
macro_rules! Depcrate_x509_teststest_save_subject_der {
() => {
// Module: crate::x509::tests
// Provides: {"test_save_subject_der"}
// Dependencies: {}
# [test] fn test_save_subject_der () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let der = cert . subject_name () . to_der () . unwrap () ; println ! ("der: {:?}" , der) ; assert ! (! der . is_empty ()) ; }
};
}
