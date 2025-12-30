// Generated macro for test_i128_len_doesnt_overflow (function)
macro_rules! Depcrate_rangetest_i128_len_doesnt_overflow {
() => {
// Module: crate::range
// Provides: {"test_i128_len_doesnt_overflow"}
// Dependencies: {}
# [test] fn test_i128_len_doesnt_overflow () { let octillion : i128 = "1000000000000000000000000000" . parse () . unwrap () ; let producer = IterProducer { range : 0 .. octillion , } ; assert_eq ! (octillion as u128 , producer . range . unindexed_len ()) ; assert_eq ! (octillion as u128 , (0 .. octillion) . unindexed_len ()) ; assert_eq ! (2 * octillion as u128 , (- octillion .. octillion) . unindexed_len ()) ; assert_eq ! (u128 :: MAX , (i128 :: MIN .. i128 :: MAX) . unindexed_len ()) ; }
};
}
