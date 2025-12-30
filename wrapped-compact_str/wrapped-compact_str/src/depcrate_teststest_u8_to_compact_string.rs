// Generated macro for test_u8_to_compact_string (function)
macro_rules! Depcrate_teststest_u8_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_u8_to_compact_string"}
// Dependencies: {}
# [test] fn test_u8_to_compact_string () { let vals = [u8 :: MIN , 1 , 42 , u8 :: MAX - 2 , u8 :: MAX - 1 , u8 :: MAX] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; assert ! (! c . is_heap_allocated ()) ; } }
};
}
