// Generated macro for test_ffdhe_params_correct (function)
macro_rules! Depcrate_validate_ffdhe_paramstest_ffdhe_params_correct {
() => {
// Module: crate::validate_ffdhe_params
// Provides: {"test_ffdhe_params_correct"}
// Dependencies: {}
fn test_ffdhe_params_correct (name : NamedGroup , group : FfdheGroup < 'static >) { let (p , g) = get_ffdhe_params_from_openssl (name) ; let openssl_params = FfdheGroup :: from_params_trimming_leading_zeros (& p , & g) ; assert_eq ! (group , openssl_params) ; }
};
}
