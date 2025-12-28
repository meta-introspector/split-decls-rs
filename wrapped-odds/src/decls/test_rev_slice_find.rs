macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! test_rev_slice_find {
    () => {
        deps!();
        # [test] fn test_rev_slice_find () { let data = [1 , 2 , 3 , 4] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; for (i , elt) in r . into_iter () . enumerate () { assert_eq ! (r . find (elt) , Some (i)) ; } for (i , elt) in r . into_iter () . enumerate () { assert_eq ! (r . rfind (elt) , Some (i)) ; } }
    };
}

test_rev_slice_find!()