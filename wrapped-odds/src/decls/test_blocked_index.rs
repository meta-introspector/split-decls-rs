macro_rules! deps {
    () => {
        BlockedIter!();
    };
}

macro_rules! test_blocked_index {
    () => {
        deps!();
        # [test] fn test_blocked_index () { let data = [0 , 1 , 2 , 3 , 4] ; let iter = BlockedIter :: < [u32 ; 2] , _ > :: from_slice (& data) ; assert_eq ! (iter [0] , [0 , 1]) ; assert_eq ! (iter [1] , [2 , 3]) ; }
    };
}

test_blocked_index!();