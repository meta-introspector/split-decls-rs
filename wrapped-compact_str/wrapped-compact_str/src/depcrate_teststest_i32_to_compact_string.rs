// Generated macro for test_i32_to_compact_string (function)
macro_rules! Depcrate_teststest_i32_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_i32_to_compact_string"}
// Dependencies: {}
# [test] fn test_i32_to_compact_string () { let vals = [i32 :: MIN , i32 :: MIN + 2 , i32 :: MIN + 1 , - 12345678 , - 42 , - 1 , 0 , 1 , 999 , 123456789 , i32 :: MAX - 2 , i32 :: MAX - 1 , i32 :: MAX ,] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; assert ! (! c . is_heap_allocated ()) ; } }
};
}
