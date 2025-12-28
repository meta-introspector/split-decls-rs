macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! test_source {
    () => {
        deps!();
        # [test] fn test_source () { use core :: error :: Error ; assert ! (UnknownCryptoError . source () . is_none ()) ; }
    };
}

test_source!()