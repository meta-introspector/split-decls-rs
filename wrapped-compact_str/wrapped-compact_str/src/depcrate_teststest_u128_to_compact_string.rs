// Generated macro for test_u128_to_compact_string (function)
macro_rules! Depcrate_teststest_u128_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_u128_to_compact_string"}
// Dependencies: {}
# [test] fn test_u128_to_compact_string () { let vals = [u128 :: MIN , 1 , 999 , 123456789 , u128 :: MAX - 2 , u128 :: MAX - 1 , u128 :: MAX ,] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; } }
};
}
