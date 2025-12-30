// Generated macro for test_subject_alt_name_iter (function)
macro_rules! Depcrate_x509_teststest_subject_alt_name_iter {
() => {
// Module: crate::x509::tests
// Provides: {"test_subject_alt_name_iter"}
// Dependencies: {}
# [test] fn test_subject_alt_name_iter () { let cert = include_bytes ! ("../../test/alt_name_cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let subject_alt_names = cert . subject_alt_names () . unwrap () ; let mut subject_alt_names_iter = subject_alt_names . iter () ; assert_eq ! (subject_alt_names_iter . next () . unwrap () . dnsname () , Some ("example.com")) ; assert_eq ! (subject_alt_names_iter . next () . unwrap () . ipaddress () , Some (& [127 , 0 , 0 , 1] [..])) ; assert_eq ! (subject_alt_names_iter . next () . unwrap () . ipaddress () , Some (& b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01" [..])) ; assert_eq ! (subject_alt_names_iter . next () . unwrap () . email () , Some ("test@example.com")) ; assert_eq ! (subject_alt_names_iter . next () . unwrap () . uri () , Some ("http://www.example.com")) ; assert ! (subject_alt_names_iter . next () . is_none ()) ; }
};
}
