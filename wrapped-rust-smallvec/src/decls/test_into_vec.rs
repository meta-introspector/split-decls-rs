macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_into_vec {
    () => {
        deps!();
        # [test] fn test_into_vec () { let vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 2) ; assert_eq ! (vec . into_vec () , vec ! [0 , 1]) ; let vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 3) ; assert_eq ! (vec . into_vec () , vec ! [0 , 1 , 2]) ; }
    };
}

test_into_vec!()