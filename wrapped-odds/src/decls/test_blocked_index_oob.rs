macro_rules! deps {
    () => {
        BlockedIter!();
    };
}

macro_rules! test_blocked_index_oob {
    () => {
        deps!();
        # [should_panic] # [test] fn test_blocked_index_oob () { let data = [0 , 1 , 2 , 3 , 4] ; let iter = BlockedIter :: < [u32 ; 2] , _ > :: from_slice (& data) ; iter [2] ; }
    };
}

test_blocked_index_oob!();