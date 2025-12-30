// Generated macro for test_unknown_crypto_from_decode_error (function)
macro_rules! Depcrate_errorstest_unknown_crypto_from_decode_error {
() => {
// Module: crate::errors
// Provides: {"test_unknown_crypto_from_decode_error"}
// Dependencies: {}
# [test] # [cfg (feature = "safe_api")] fn test_unknown_crypto_from_decode_error () { use ct_codecs :: Error ; let err_one = Error :: InvalidInput ; let err_two = Error :: Overflow ; let err = format ! ("{:?}:{}" , UnknownCryptoError :: from (err_one) , UnknownCryptoError :: from (err_one)) ; assert_eq ! (err , "UnknownCryptoError:UnknownCryptoError") ; let err = format ! ("{:?}:{}" , UnknownCryptoError :: from (err_two) , UnknownCryptoError :: from (err_two)) ; assert_eq ! (err , "UnknownCryptoError:UnknownCryptoError") ; }
};
}
