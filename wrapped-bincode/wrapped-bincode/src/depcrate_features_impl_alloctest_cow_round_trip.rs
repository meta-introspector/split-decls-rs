// Generated macro for test_cow_round_trip (function)
macro_rules! Depcrate_features_impl_alloctest_cow_round_trip {
() => {
// Module: crate::features::impl_alloc
// Provides: {"test_cow_round_trip"}
// Dependencies: {}
# [test] fn test_cow_round_trip () { let start = Cow :: Borrowed ("Foo") ; let encoded = crate :: encode_to_vec (& start , crate :: config :: standard ()) . unwrap () ; let (end , _) = crate :: borrow_decode_from_slice :: < Cow < str > , _ > (& encoded , crate :: config :: standard ()) . unwrap () ; assert_eq ! (start , end) ; let (end , _) = crate :: decode_from_slice :: < Cow < str > , _ > (& encoded , crate :: config :: standard ()) . unwrap () ; assert_eq ! (start , end) ; }
};
}
