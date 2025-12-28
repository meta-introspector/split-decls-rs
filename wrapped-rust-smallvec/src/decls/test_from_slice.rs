macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_from_slice {
    () => {
        deps!();
        # [test] fn test_from_slice () { assert_eq ! (& SmallVec ::< u32 , 2 >:: from (& [1] [..]) [..] , [1]) ; assert_eq ! (& SmallVec ::< u32 , 2 >:: from (& [1 , 2 , 3] [..]) [..] , [1 , 2 , 3]) ; }
    };
}

test_from_slice!();