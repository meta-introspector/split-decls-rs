// Generated macro for test_verify_param_set_depth_fails_verification (function)
macro_rules! Depcrate_x509_teststest_verify_param_set_depth_fails_verification {
() => {
// Module: crate::x509::tests
// Provides: {"test_verify_param_set_depth_fails_verification"}
// Dependencies: {}
# [test] # [allow (clippy :: bool_to_int_with_if)] fn test_verify_param_set_depth_fails_verification () { let cert = include_bytes ! ("../../test/leaf.pem") ; let cert = X509 :: from_pem (cert) . unwrap () ; let intermediate_ca = include_bytes ! ("../../test/intermediate-ca.pem") ; let intermediate_ca = X509 :: from_pem (intermediate_ca) . unwrap () ; let ca = include_bytes ! ("../../test/root-ca.pem") ; let ca = X509 :: from_pem (ca) . unwrap () ; let mut chain = Stack :: new () . unwrap () ; chain . push (intermediate_ca) . unwrap () ; let mut store_bldr = X509StoreBuilder :: new () . unwrap () ; store_bldr . add_cert (ca) . unwrap () ; let mut verify_params = X509VerifyParam :: new () . unwrap () ; let expected_depth = if cfg ! (any (ossl110 , boringssl , awslc)) { 0 } else { 1 } ; verify_params . set_depth (expected_depth) ; store_bldr . set_param (& verify_params) . unwrap () ; let store = store_bldr . build () ; let expected_error = if cfg ! (any (ossl110 , libressl)) { "certificate chain too long" } else { "unable to get local issuer certificate" } ; let mut context = X509StoreContext :: new () . unwrap () ; assert_eq ! (context . init (& store , & cert , & chain , | c | { c . verify_cert () ?; Ok (c . error ()) }) . unwrap () . error_string () , expected_error) }
};
}
