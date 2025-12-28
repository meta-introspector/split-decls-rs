macro_rules! deps {
    () => {
        PrefixRange!();
    };
}

macro_rules! test_prefix_range {
    () => {
        deps!();
        # [test] fn test_prefix_range () { fn test (start : & [u8] , end : Option < & [u8] >) { let got = PrefixRange (start) . into_bounds () ; assert_eq ! ((Some (start) , end) , (got . 0 . as_deref () , got . 1 . as_deref ())) ; } let empty : & [u8] = & [] ; assert_eq ! ((None , None) , PrefixRange (empty) . into_bounds ()) ; test (b"\xff" , None) ; test (b"\xff\xff\xff\xff" , None) ; test (b"a" , Some (b"b")) ; test (b"a\xff\xff\xff" , Some (b"b")) ; }
    };
}

test_prefix_range!()