// Generated macro for test_nid_uid_value (function)
macro_rules! Depcrate_x509_teststest_nid_uid_value {
() => {
// Module: crate::x509::tests
// Provides: {"test_nid_uid_value"}
// Dependencies: {}
# [test] fn test_nid_uid_value () { let cert = include_bytes ! ("../../test/nid_uid_test_cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let subject = cert . subject_name () ; let cn = subject . entries_by_nid (Nid :: USERID) . next () . unwrap () ; assert_eq ! (cn . data () . as_slice () , b"this is the userId") ; }
};
}
