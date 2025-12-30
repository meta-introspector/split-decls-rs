// Generated macro for test_i16_to_compact_string (function)
macro_rules! Depcrate_teststest_i16_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_i16_to_compact_string"}
// Dependencies: {}
# [test] fn test_i16_to_compact_string () { let vals = [i16 :: MIN , i16 :: MIN + 1 , i16 :: MIN + 2 , - 42 , - 1 , 0 , 1 , 42 , 999 , i16 :: MAX - 2 , i16 :: MAX - 1 , i16 :: MAX ,] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; assert ! (! c . is_heap_allocated ()) ; } }
};
}
