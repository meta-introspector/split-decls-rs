macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! test_rev_slice_hash {
    () => {
        deps!();
        # [test] fn test_rev_slice_hash () { let data = [1 , 2 , 3 , 4] ; let rev = [4 , 3 , 2 , 1] ; let r = < & RevSlice < _ > > :: from (& data [..]) ; # [allow (deprecated)] fn hash < T : ? Sized + Hash > (value : & T) -> u64 { use std :: hash :: SipHasher ; let mut h = SipHasher :: new () ; value . hash (& mut h) ; h . finish () } for i in 0 .. r . len () { for j in i .. r . len () { assert_eq ! (hash (& r [i .. j]) , hash (& rev [i .. j])) ; } } }
    };
}

test_rev_slice_hash!()