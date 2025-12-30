// Generated macro for test_nameref_iterator (function)
macro_rules! Depcrate_x509_teststest_nameref_iterator {
() => {
// Module: crate::x509::tests
// Provides: {"test_nameref_iterator"}
// Dependencies: {}
# [test] fn test_nameref_iterator () { let cert = include_bytes ! ("../../test/nid_test_cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let subject = cert . subject_name () ; let mut all_entries = subject . entries () ; let email = all_entries . next () . unwrap () ; assert_eq ! (email . object () . nid () . as_raw () , Nid :: PKCS9_EMAILADDRESS . as_raw ()) ; assert_eq ! (email . data () . as_slice () , b"test@example.com") ; let cn = all_entries . next () . unwrap () ; assert_eq ! (cn . object () . nid () . as_raw () , Nid :: COMMONNAME . as_raw ()) ; assert_eq ! (cn . data () . as_slice () , b"example.com") ; let friendly = all_entries . next () . unwrap () ; assert_eq ! (friendly . object () . nid () . as_raw () , Nid :: FRIENDLYNAME . as_raw ()) ; assert_eq ! (&** friendly . data () . as_utf8 () . unwrap () , "Example") ; if all_entries . next () . is_some () { panic ! () ; } }
};
}
