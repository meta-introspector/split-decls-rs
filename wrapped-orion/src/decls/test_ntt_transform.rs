macro_rules! deps {
    () => {
        RingElementNTT!();
        RingElement!();
    };
}

macro_rules! test_ntt_transform {
    () => {
        deps!();
        # [cfg (all (test , feature = "safe_api"))] mod test_ntt_transform { use super :: * ; # [test] fn test_to_from_ntt_roundtrips () { for _ in 0 .. 100 { let f : RingElement = RingElement :: random_element () ; let f_hat : RingElementNTT = to_ntt (& RingElement :: random_element ()) ; assert_eq ! (f , inverse_ntt (& to_ntt (& f)) ,) ; assert_eq ! (f_hat , to_ntt (& inverse_ntt (& f_hat)) ,) ; } } }
    };
}

test_ntt_transform!()