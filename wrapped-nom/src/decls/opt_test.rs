macro_rules! deps {
    () => {
        Needed!();
        Err!();
        IResult!();
    };
}

macro_rules! opt_test {
    () => {
        deps!();
        # [test] fn opt_test () { fn opt_abcd (i : & [u8]) -> IResult < & [u8] , Option < & [u8] > > { opt (tag ("abcd")) . parse (i) } let a = & b"abcdef" [..] ; let b = & b"bcdefg" [..] ; let c = & b"ab" [..] ; assert_eq ! (opt_abcd (a) , Ok ((& b"ef" [..] , Some (& b"abcd" [..])))) ; assert_eq ! (opt_abcd (b) , Ok ((& b"bcdefg" [..] , None))) ; assert_eq ! (opt_abcd (c) , Err (Err :: Incomplete (Needed :: new (2)))) ; }
    };
}

opt_test!();