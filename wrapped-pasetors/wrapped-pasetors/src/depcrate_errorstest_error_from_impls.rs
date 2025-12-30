// Generated macro for test_error_from_impls (function)
macro_rules! Depcrate_errorstest_error_from_impls {
() => {
// Module: crate::errors
// Provides: {"test_error_from_impls"}
// Dependencies: {}
# [test] fn test_error_from_impls () { let _ = format ! ("{:?}" , Error :: TokenFormat) ; let _ = format ! ("{}" , Error :: TokenFormat) ; assert_eq ! (Error :: from (ct_codecs :: Error :: InvalidInput) , Error :: Base64) ; assert_eq ! (Error :: from (getrandom :: Error :: UNSUPPORTED) , Error :: Csprng) ; }
};
}
