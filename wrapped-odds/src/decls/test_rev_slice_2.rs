macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! test_rev_slice_2 {
    () => {
        deps!();
        # [should_panic] # [test] fn test_rev_slice_2 () { let data = [1 , 2 , 3 , 4] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; r [4] ; }
    };
}

test_rev_slice_2!()