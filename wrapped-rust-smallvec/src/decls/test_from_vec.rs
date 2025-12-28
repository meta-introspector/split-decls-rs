macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_from_vec {
    () => {
        deps!();
        # [test] fn test_from_vec () { let vec = vec ! [] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & []) ; drop (small_vec) ; let vec = vec ! [] ; let small_vec : SmallVec < u8 , 1 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & []) ; drop (small_vec) ; let vec = vec ! [1] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & [1]) ; drop (small_vec) ; let vec = vec ! [1 , 2 , 3] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & [1 , 2 , 3]) ; drop (small_vec) ; let vec = vec ! [1 , 2 , 3 , 4 , 5] ; let small_vec : SmallVec < u8 , 3 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & [1 , 2 , 3 , 4 , 5]) ; drop (small_vec) ; let vec = vec ! [1 , 2 , 3 , 4 , 5] ; let small_vec : SmallVec < u8 , 1 > = SmallVec :: from_vec (vec) ; assert_eq ! (&* small_vec , & [1 , 2 , 3 , 4 , 5]) ; drop (small_vec) ; }
    };
}

test_from_vec!();