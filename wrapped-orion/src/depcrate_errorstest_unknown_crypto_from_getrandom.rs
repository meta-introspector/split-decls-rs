// Generated macro for test_unknown_crypto_from_getrandom (function)
macro_rules! Depcrate_errorstest_unknown_crypto_from_getrandom {
() => {
// Module: crate::errors
// Provides: {"test_unknown_crypto_from_getrandom"}
// Dependencies: {}
# [test] # [cfg (feature = "safe_api")] fn test_unknown_crypto_from_getrandom () { let err_code : u16 = 12 ; let err_foreign : getrandom :: Error = getrandom :: Error :: new_custom (err_code) ; let err = format ! ("{:?}" , UnknownCryptoError :: from (err_foreign)) ; assert_eq ! (err , "UnknownCryptoError") ; let err = format ! ("{}" , UnknownCryptoError :: from (err_foreign)) ; assert_eq ! (err , "UnknownCryptoError") ; }
};
}
