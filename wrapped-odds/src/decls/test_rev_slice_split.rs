macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! test_rev_slice_split {
    () => {
        deps!();
        # [test] fn test_rev_slice_split () { let data = [1 , 2 , 3 , 4] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; for i in 0 .. r . len () { let (a , b) = r . split_at (i) ; assert_eq ! (a , & r [.. i]) ; assert_eq ! (b , & r [i ..]) ; } }
    };
}

test_rev_slice_split!()