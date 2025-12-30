// Generated macro for test_subject_read_cn (function)
macro_rules! Depcrate_x509_teststest_subject_read_cn {
() => {
// Module: crate::x509::tests
// Provides: {"test_subject_read_cn"}
// Dependencies: {}
# [test] fn test_subject_read_cn () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let subject = cert . subject_name () ; let cn = subject . entries_by_nid (Nid :: COMMONNAME) . next () . unwrap () ; assert_eq ! (cn . data () . as_slice () , b"foobar.com") }
};
}
