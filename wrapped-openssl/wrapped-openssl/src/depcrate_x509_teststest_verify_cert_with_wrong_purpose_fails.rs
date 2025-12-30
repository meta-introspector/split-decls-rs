// Generated macro for test_verify_cert_with_wrong_purpose_fails (function)
macro_rules! Depcrate_x509_teststest_verify_cert_with_wrong_purpose_fails {
() => {
// Module: crate::x509::tests
// Provides: {"test_verify_cert_with_wrong_purpose_fails"}
// Dependencies: {}
# [test] fn test_verify_cert_with_wrong_purpose_fails () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let ca = include_bytes ! ("../../test/root-ca.pem") ; let ca = X509 :: from_pem (ca) . unwrap () ; let chain = Stack :: new () . unwrap () ; let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; let purpose_idx = X509PurposeRef :: get_by_sname ("timestampsign") . expect ("Getting certificate purpose 'timestampsign' failed") ; let x509_purpose = X509PurposeRef :: from_idx (purpose_idx) . expect ("Getting certificate purpose failed") ; store_bldr . set_purpose (x509_purpose . purpose ()) . expect ("Setting certificate purpose failed") ; store_bldr . add_cert (ca) . unwrap () ; let store = store_bldr . build () ; let expected_error = ffi :: X509_V_ERR_INVALID_PURPOSE ; let mut context = X509StoreContext :: new () . unwrap () ; assert_eq ! (context . init (& store , & cert , & chain , | c | { c . verify_cert () ?; Ok (c . error ()) }) . unwrap () . as_raw () , expected_error) }
};
}
