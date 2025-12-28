macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! test_unknown_crypto_error_debug_display {
    () => {
        deps!();
        # [test] # [cfg (feature = "safe_api")] fn test_unknown_crypto_error_debug_display () { let err = format ! ("{UnknownCryptoError:?}") ; assert_eq ! (err , "UnknownCryptoError") ; let err = format ! ("{UnknownCryptoError}") ; assert_eq ! (err , "UnknownCryptoError") ; }
    };
}

test_unknown_crypto_error_debug_display!()