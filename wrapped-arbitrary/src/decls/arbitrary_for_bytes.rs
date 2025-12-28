macro_rules! deps {
    () => {
        Unstructured!();
    };
}

macro_rules! arbitrary_for_bytes {
    () => {
        deps!();
        # [test] fn arbitrary_for_bytes () { let x = [1 , 2 , 3 , 4 , 4] ; let mut buf = Unstructured :: new (& x) ; let expected = & [1 , 2 , 3 , 4] ; let actual = checked_arbitrary :: < & [u8] > (& mut buf) . unwrap () ; assert_eq ! (expected , actual) ; }
    };
}

arbitrary_for_bytes!()