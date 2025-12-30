// Generated macro for test_i128_to_compact_string (function)
macro_rules! Depcrate_teststest_i128_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_i128_to_compact_string"}
// Dependencies: {}
# [test] fn test_i128_to_compact_string () { let vals = [i128 :: MIN , i128 :: MIN + 1 , i128 :: MIN + 2 , - 22222222 , - 42 , 0 , 1 , 999 , 123456789 , i128 :: MAX - 2 , i128 :: MAX - 1 , i128 :: MAX ,] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; } }
};
}
