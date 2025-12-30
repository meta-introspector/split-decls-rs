// Generated macro for issued (function)
macro_rules! Depcrate_x509_testsissued {
() => {
// Module: crate::x509::tests
// Provides: {"issued"}
// Dependencies: {}
# [test] fn issued () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let ca = include_bytes ! ("../../test/root-ca.pem") ; let ca = X509 :: from_pem (ca) . unwrap () ; assert_eq ! (ca . issued (& cert) , X509VerifyResult :: OK) ; assert_ne ! (cert . issued (& cert) , X509VerifyResult :: OK) ; }
};
}
