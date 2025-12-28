macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! test_unknown_crypto_from_getrandom {
    () => {
        deps!();
        # [test] # [cfg (feature = "safe_api")] fn test_unknown_crypto_from_getrandom () { let err_code : u16 = 12 ; let err_foreign : getrandom :: Error = getrandom :: Error :: new_custom (err_code) ; let err = format ! ("{:?}" , UnknownCryptoError :: from (err_foreign)) ; assert_eq ! (err , "UnknownCryptoError") ; let err = format ! ("{}" , UnknownCryptoError :: from (err_foreign)) ; assert_eq ! (err , "UnknownCryptoError") ; }
    };
}

test_unknown_crypto_from_getrandom!();