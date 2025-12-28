macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! test_rev_slice_1 {
    () => {
        deps!();
        # [test] fn test_rev_slice_1 () { let data = [1 , 2 , 3 , 4] ; let rev = [4 , 3 , 2 , 1] ; assert_eq ! (<& RevSlice < _ >>:: from (& data [..]) , & rev [..]) ; assert ! (<& RevSlice < _ >>:: from (& data [..]) != & data [..]) ; let r = < & RevSlice < _ > > :: from (& data [..]) ; assert_eq ! (r [0] , rev [0]) ; assert_eq ! (r [3] , rev [3]) ; }
    };
}

test_rev_slice_1!()