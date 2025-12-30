// Generated macro for test_verify_fails_with_crl_flag_set_and_no_crl (function)
macro_rules! Depcrate_x509_teststest_verify_fails_with_crl_flag_set_and_no_crl {
() => {
// Module: crate::x509::tests
// Provides: {"test_verify_fails_with_crl_flag_set_and_no_crl"}
// Dependencies: {}
# [test] fn test_verify_fails_with_crl_flag_set_and_no_crl () { let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let ca = include_bytes ! ("../../test/root-ca.pem") ; let ca = X509 :: from_pem (ca) . unwrap () ; let chain = Stack :: new () . unwrap () ; let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; store_bldr . add_cert (ca) . unwrap () ; store_bldr . set_flags (X509VerifyFlags :: CRL_CHECK) . unwrap () ; let store = store_bldr . build () ; let mut context = X509StoreContext :: new () . unwrap () ; assert_eq ! (context . init (& store , & cert , & chain , | c | { c . verify_cert () ?; Ok (c . error ()) }) . unwrap () . error_string () , "unable to get certificate CRL") }
};
}
