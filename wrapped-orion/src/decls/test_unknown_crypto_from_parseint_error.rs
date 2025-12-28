macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! test_unknown_crypto_from_parseint_error {
    () => {
        deps!();
        # [test] # [cfg (feature = "safe_api")] fn test_unknown_crypto_from_parseint_error () { let err_foreign = "j" . parse :: < u32 > () . unwrap_err () ; let err = format ! ("{:?}:{}" , UnknownCryptoError :: from (err_foreign . clone ()) , UnknownCryptoError :: from (err_foreign)) ; assert_eq ! (err , "UnknownCryptoError:UnknownCryptoError") ; }
    };
}

test_unknown_crypto_from_parseint_error!()