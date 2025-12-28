macro_rules! deps {
    () => {
        Unstructured!();
    };
}

macro_rules! arbitrary_take_rest_for_bytes {
    () => {
        deps!();
        # [test] fn arbitrary_take_rest_for_bytes () { let x = [1 , 2 , 3 , 4] ; let buf = Unstructured :: new (& x) ; let expected = & [1 , 2 , 3 , 4] ; let actual = checked_arbitrary_take_rest :: < & [u8] > (buf) . unwrap () ; assert_eq ! (expected , actual) ; }
    };
}

arbitrary_take_rest_for_bytes!();