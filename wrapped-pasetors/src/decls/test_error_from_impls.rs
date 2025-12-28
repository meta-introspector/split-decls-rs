macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! test_error_from_impls {
    () => {
        deps!();
        # [test] fn test_error_from_impls () { let _ = format ! ("{:?}" , Error :: TokenFormat) ; let _ = format ! ("{}" , Error :: TokenFormat) ; assert_eq ! (Error :: from (ct_codecs :: Error :: InvalidInput) , Error :: Base64) ; assert_eq ! (Error :: from (getrandom :: Error :: UNSUPPORTED) , Error :: Csprng) ; }
    };
}

test_error_from_impls!()