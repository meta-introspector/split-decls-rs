// Generated macro for test_debug (function)
macro_rules! Depcrate_x509_teststest_debug {
() => {
// Module: crate::x509::tests
// Provides: {"test_debug"}
// Dependencies: {}
# [test] fn test_debug () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let debugged = format ! ("{:#?}" , cert) ; assert ! (debugged . contains (r#"serial_number: "8771F7BDEE982FA5""#) || debugged . contains (r#"serial_number: "8771f7bdee982fa5""#)) ; assert ! (debugged . contains (r#"signature_algorithm: sha256WithRSAEncryption"#)) ; assert ! (debugged . contains (r#"countryName = "AU""#)) ; assert ! (debugged . contains (r#"stateOrProvinceName = "Some-State""#)) ; assert ! (debugged . contains (r#"not_before: Aug 14 17:00:03 2016 GMT"#)) ; assert ! (debugged . contains (r#"not_after: Aug 12 17:00:03 2026 GMT"#)) ; }
};
}
