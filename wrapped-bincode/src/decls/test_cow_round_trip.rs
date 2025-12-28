macro_rules! test_cow_round_trip {
    () => {
        # [test] fn test_cow_round_trip () { let start = Cow :: Borrowed ("Foo") ; let encoded = crate :: encode_to_vec (& start , crate :: config :: standard ()) . unwrap () ; let (end , _) = crate :: borrow_decode_from_slice :: < Cow < str > , _ > (& encoded , crate :: config :: standard ()) . unwrap () ; assert_eq ! (start , end) ; let (end , _) = crate :: decode_from_slice :: < Cow < str > , _ > (& encoded , crate :: config :: standard ()) . unwrap () ; assert_eq ! (start , end) ; }
    };
}

test_cow_round_trip!();