// Generated macro for test_name_to_owned (function)
macro_rules! Depcrate_x509_teststest_name_to_owned {
() => {
// Module: crate::x509::tests
// Provides: {"test_name_to_owned"}
// Dependencies: {}
# [test] # [cfg (any (boringssl , ossl110 , libressl , awslc))] fn test_name_to_owned () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let name = cert . subject_name () ; let copied_name = name . to_owned () . unwrap () ; assert_eq ! (Ordering :: Equal , name . try_cmp (& copied_name) . unwrap ()) ; }
};
}
