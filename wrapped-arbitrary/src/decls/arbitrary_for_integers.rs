macro_rules! deps {
    () => {
        Unstructured!();
    };
}

macro_rules! arbitrary_for_integers {
    () => {
        deps!();
        # [test] fn arbitrary_for_integers () { let x = [1 , 2 , 3 , 4] ; let mut buf = Unstructured :: new (& x) ; let expected = 1 | (2 << 8) | (3 << 16) | (4 << 24) ; let actual = checked_arbitrary :: < i32 > (& mut buf) . unwrap () ; assert_eq ! (expected , actual) ; assert_generates ([i32 :: from_ne_bytes ([0 , 0 , 0 , 0]) , i32 :: from_ne_bytes ([0 , 0 , 0 , 1]) , i32 :: from_ne_bytes ([0 , 0 , 1 , 0]) , i32 :: from_ne_bytes ([0 , 1 , 0 , 0]) , i32 :: from_ne_bytes ([1 , 0 , 0 , 0]) , i32 :: from_ne_bytes ([1 , 1 , 1 , 1]) , i32 :: from_ne_bytes ([0xff , 0xff , 0xff , 0xff]) ,]) ; }
    };
}

arbitrary_for_integers!();