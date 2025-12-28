macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! test_cows_regression {
    () => {
        deps!();
        # [test] # [cfg (feature = "std")] fn test_cows_regression () { use std :: borrow :: Cow ; use crate :: ByteSlice ; let c1 = Cow :: from (b"hello bstr" . as_bstr ()) ; let c2 = b"goodbye bstr" . as_bstr () ; assert_ne ! (c1 , c2) ; let c3 = Cow :: from ("hello str") ; let c4 = "goodbye str" ; assert_ne ! (c3 , c4) ; }
    };
}

test_cows_regression!()