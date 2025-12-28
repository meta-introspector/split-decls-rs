macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! test_rev_slice_3 {
    () => {
        deps!();
        # [should_panic] # [test] fn test_rev_slice_3 () { let data = [1 , 2 , 3 , 4] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; r [! 0] ; }
    };
}

test_rev_slice_3!()