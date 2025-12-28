macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! test_rev_slice_slice {
    () => {
        deps!();
        # [test] fn test_rev_slice_slice () { let data = [1 , 2 , 3 , 4] ; let rev = [4 , 3 , 2 , 1] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; for i in 0 .. r . len () { for j in i .. r . len () { assert_eq ! (& r [i .. j] , & rev [i .. j]) ; } } }
    };
}

test_rev_slice_slice!()