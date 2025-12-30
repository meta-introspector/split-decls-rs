// Generated macro for test_verify_cert_with_purpose (function)
macro_rules! Depcrate_x509_teststest_verify_cert_with_purpose {
() => {
// Module: crate::x509::tests
// Provides: {"test_verify_cert_with_purpose"}
// Dependencies: {}
# [test] fn test_verify_cert_with_purpose () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let ca = include_bytes ! ("../../test/root-ca.pem") ; let ca = X509 :: from_pem (ca) . unwrap () ; let chain = Stack :: new () . unwrap () ; let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; let purpose_idx = X509PurposeRef :: get_by_sname ("sslserver") . expect ("Getting certificate purpose 'sslserver' failed") ; let x509_purposeref = X509PurposeRef :: from_idx (purpose_idx) . expect ("Getting certificate purpose failed") ; store_bldr . set_purpose (x509_purposeref . purpose ()) . expect ("Setting certificate purpose failed") ; store_bldr . add_cert (ca) . unwrap () ; let store = store_bldr . build () ; let mut context = X509StoreContext :: new () . unwrap () ; assert ! (context . init (& store , & cert , & chain , | c | c . verify_cert ()) . unwrap ()) ; }
};
}
