macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_into_inner {
    () => {
        deps!();
        # [test] fn test_into_inner () { let vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 2) ; assert_eq ! (vec . into_inner () , Ok ([0 , 1])) ; let vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 1) ; assert_eq ! (vec . clone () . into_inner () , Err (vec)) ; let vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 3) ; assert_eq ! (vec . clone () . into_inner () , Err (vec)) ; }
    };
}

test_into_inner!();