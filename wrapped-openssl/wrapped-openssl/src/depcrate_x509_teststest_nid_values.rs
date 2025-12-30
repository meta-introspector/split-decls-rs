// Generated macro for test_nid_values (function)
macro_rules! Depcrate_x509_teststest_nid_values {
() => {
// Module: crate::x509::tests
// Provides: {"test_nid_values"}
// Dependencies: {}
# [test] fn test_nid_values () { let cert = include_bytes ! ("../../test/nid_test_cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let subject = cert . subject_name () ; let cn = subject . entries_by_nid (Nid :: COMMONNAME) . next () . unwrap () ; assert_eq ! (cn . data () . as_slice () , b"example.com") ; let email = subject . entries_by_nid (Nid :: PKCS9_EMAILADDRESS) . next () . unwrap () ; assert_eq ! (email . data () . as_slice () , b"test@example.com") ; let friendly = subject . entries_by_nid (Nid :: FRIENDLYNAME) . next () . unwrap () ; assert_eq ! (&** friendly . data () . as_utf8 () . unwrap () , "Example") ; }
};
}
