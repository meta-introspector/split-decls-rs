// Generated macro for test_verify_param_auth_level (function)
macro_rules! Depcrate_x509_teststest_verify_param_auth_level {
() => {
// Module: crate::x509::tests
// Provides: {"test_verify_param_auth_level"}
// Dependencies: {}
# [test] # [cfg (ossl110)] fn test_verify_param_auth_level () { let mut param = X509VerifyParam :: new () . unwrap () ; let auth_lvl = 2 ; let auth_lvl_default = - 1 ; assert_eq ! (param . auth_level () , auth_lvl_default) ; param . set_auth_level (auth_lvl) ; assert_eq ! (param . auth_level () , auth_lvl) ; }
};
}
