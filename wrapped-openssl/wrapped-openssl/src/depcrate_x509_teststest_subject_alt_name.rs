// Generated macro for test_subject_alt_name (function)
macro_rules! Depcrate_x509_teststest_subject_alt_name {
() => {
// Module: crate::x509::tests
// Provides: {"test_subject_alt_name"}
// Dependencies: {}
# [test] fn test_subject_alt_name () { let cert = include_bytes ! ("../../test/alt_name_cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let subject_alt_names = cert . subject_alt_names () . unwrap () ; assert_eq ! (5 , subject_alt_names . len ()) ; assert_eq ! (Some ("example.com") , subject_alt_names [0] . dnsname ()) ; assert_eq ! (subject_alt_names [1] . ipaddress () , Some (& [127 , 0 , 0 , 1] [..])) ; assert_eq ! (subject_alt_names [2] . ipaddress () , Some (& b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01" [..])) ; assert_eq ! (Some ("test@example.com") , subject_alt_names [3] . email ()) ; assert_eq ! (Some ("http://www.example.com") , subject_alt_names [4] . uri ()) ; }
};
}
