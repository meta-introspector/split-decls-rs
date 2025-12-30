// Generated macro for test_cert_issue_validity (function)
macro_rules! Depcrate_x509_teststest_cert_issue_validity {
() => {
// Module: crate::x509::tests
// Provides: {"test_cert_issue_validity"}
// Dependencies: {}
# [test] fn test_cert_issue_validity () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let not_before = cert . not_before () . to_string () ; let not_after = cert . not_after () . to_string () ; assert_eq ! (not_before , "Aug 14 17:00:03 2016 GMT") ; assert_eq ! (not_after , "Aug 12 17:00:03 2026 GMT") ; }
};
}
