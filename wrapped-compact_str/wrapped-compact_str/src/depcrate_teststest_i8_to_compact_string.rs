// Generated macro for test_i8_to_compact_string (function)
macro_rules! Depcrate_teststest_i8_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_i8_to_compact_string"}
// Dependencies: {}
# [test] fn test_i8_to_compact_string () { let vals = [i8 :: MIN , i8 :: MIN + 1 , i8 :: MIN + 2 , - 1 , 0 , 1 , 42 , i8 :: MAX - 2 , i8 :: MAX - 1 , i8 :: MAX ,] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; assert ! (! c . is_heap_allocated ()) ; } }
};
}
