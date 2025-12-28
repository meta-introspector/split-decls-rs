macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_from {
    () => {
        deps!();
        # [test] fn test_from () { assert_eq ! (& SmallVec ::< u32 , 2 >:: from (& [1] [..]) [..] , [1]) ; assert_eq ! (& SmallVec ::< u32 , 2 >:: from (& [1 , 2 , 3] [..]) [..] , [1 , 2 , 3]) ; let vec = vec ! [] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from (vec) ; assert_eq ! (&* small_vec , & []) ; drop (small_vec) ; let vec = vec ! [1 , 2 , 3 , 4 , 5] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from (vec) ; assert_eq ! (&* small_vec , & [1 , 2 , 3 , 4 , 5]) ; drop (small_vec) ; let vec = vec ! [1 , 2 , 3 , 4 , 5] ; let small_vec : SmallVec < u8 , 1 > = SmallVec :: from (vec) ; assert_eq ! (&* small_vec , & [1 , 2 , 3 , 4 , 5]) ; drop (small_vec) ; let array = [1] ; let small_vec : SmallVec < u8 , 1 > = SmallVec :: from (array) ; assert_eq ! (&* small_vec , & [1]) ; drop (small_vec) ; let array = [99 ; 128] ; let small_vec : SmallVec < u8 , 128 > = SmallVec :: from (array) ; assert_eq ! (&* small_vec , vec ! [99u8 ; 128] . as_slice ()) ; drop (small_vec) ; # [derive (PartialEq , Eq , Debug)] struct NoClone (u8) ; let array = [NoClone (42)] ; let small_vec : SmallVec < NoClone , 1 > = SmallVec :: from (array) ; assert_eq ! (&* small_vec , & [NoClone (42)]) ; drop (small_vec) ; let vec = vec ! [NoClone (42)] ; let small_vec : SmallVec < NoClone , 1 > = SmallVec :: from (vec) ; assert_eq ! (&* small_vec , & [NoClone (42)]) ; drop (small_vec) ; let array = [1 ; 128] ; let small_vec : SmallVec < u8 , 1 > = SmallVec :: from (array) ; assert_eq ! (&* small_vec , vec ! [1 ; 128] . as_slice ()) ; drop (small_vec) ; let array = [99] ; let small_vec : SmallVec < u8 , 128 > = SmallVec :: from (array) ; assert_eq ! (&* small_vec , & [99u8]) ; drop (small_vec) ; }
    };
}

test_from!()