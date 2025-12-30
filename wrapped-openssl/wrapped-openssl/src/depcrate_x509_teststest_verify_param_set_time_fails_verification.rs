// Generated macro for test_verify_param_set_time_fails_verification (function)
macro_rules! Depcrate_x509_teststest_verify_param_set_time_fails_verification {
() => {
// Module: crate::x509::tests
// Provides: {"test_verify_param_set_time_fails_verification"}
// Dependencies: {}
# [test] fn test_verify_param_set_time_fails_verification () { const TEST_T_2030 : time_t = 1893456000 ; let cert = include_bytes ! ("../../test/cert.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let ca = include_bytes ! ("../../test/root-ca.pem") ; let ca = X509 :: from_pem (ca) . unwrap () ; let chain = Stack :: new () . unwrap () ; let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; store_bldr . add_cert (ca) . unwrap () ; let mut verify_params = X509VerifyParam :: new () . unwrap () ; verify_params . set_time (TEST_T_2030) ; store_bldr . set_param (& verify_params) . unwrap () ; let store = store_bldr . build () ; let mut context = X509StoreContext :: new () . unwrap () ; assert_eq ! (context . init (& store , & cert , & chain , | c | { c . verify_cert () ?; Ok (c . error ()) }) . unwrap () . error_string () , "certificate has expired") }
};
}
