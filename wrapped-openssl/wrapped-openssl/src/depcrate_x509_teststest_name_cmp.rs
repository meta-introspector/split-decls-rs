// Generated macro for test_name_cmp (function)
macro_rules! Depcrate_x509_teststest_name_cmp {
() => {
// Module: crate::x509::tests
// Provides: {"test_name_cmp"}
// Dependencies: {}
# [test] fn test_name_cmp () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let subject = cert . subject_name () ; let issuer = cert . issuer_name () ; assert_eq ! (Ordering :: Equal , subject . try_cmp (subject) . unwrap ()) ; assert_eq ! (Ordering :: Greater , subject . try_cmp (issuer) . unwrap ()) ; }
};
}
