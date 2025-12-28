macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_into_iter_as_slice {
    () => {
        deps!();
        # [test] fn test_into_iter_as_slice () { let vec = SmallVec :: < u32 , 2 > :: from (& [1 , 2 , 3] [..]) ; let mut iter = vec . clone () . into_iter () ; assert_eq ! (iter . as_slice () , & [1 , 2 , 3]) ; assert_eq ! (iter . as_mut_slice () , & [1 , 2 , 3]) ; iter . next () ; assert_eq ! (iter . as_slice () , & [2 , 3]) ; assert_eq ! (iter . as_mut_slice () , & [2 , 3]) ; iter . next_back () ; assert_eq ! (iter . as_slice () , & [2]) ; assert_eq ! (iter . as_mut_slice () , & [2]) ; }
    };
}

test_into_iter_as_slice!();