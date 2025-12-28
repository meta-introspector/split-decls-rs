macro_rules! deps {
    () => {
        BlockedIter!();
    };
}

macro_rules! test_blocked {
    () => {
        deps!();
        # [test] fn test_blocked () { let data = [0 , 1 , 2 , 3 , 4] ; let mut iter = BlockedIter :: < [u32 ; 2] , _ > :: from_slice (& data) ; assert_eq ! (iter . next () , Some (& [0 , 1])) ; assert_eq ! (iter . next () , Some (& [2 , 3])) ; assert_eq ! (iter . next () , None) ; assert_eq ! (iter . tail () . as_slice () , & [4]) ; let mut iter = BlockedIter :: < [u32 ; 8] , _ > :: from_slice (& data) ; assert_eq ! (iter . next () , None) ; assert_eq ! (iter . tail () . as_slice () , & data) ; }
    };
}

test_blocked!();