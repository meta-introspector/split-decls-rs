// Generated macro for test_unknown_crypto_error_debug_display (function)
macro_rules! Depcrate_errorstest_unknown_crypto_error_debug_display {
() => {
// Module: crate::errors
// Provides: {"test_unknown_crypto_error_debug_display"}
// Dependencies: {}
# [test] # [cfg (feature = "safe_api")] fn test_unknown_crypto_error_debug_display () { let err = format ! ("{UnknownCryptoError:?}") ; assert_eq ! (err , "UnknownCryptoError") ; let err = format ! ("{UnknownCryptoError}") ; assert_eq ! (err , "UnknownCryptoError") ; }
};
}
