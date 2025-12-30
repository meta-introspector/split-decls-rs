// Generated macro for test_u32_to_compact_string (function)
macro_rules! Depcrate_teststest_u32_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_u32_to_compact_string"}
// Dependencies: {}
# [test] fn test_u32_to_compact_string () { let vals = [u32 :: MIN , 1 , 42 , 999 , 123456789 , u32 :: MAX - 2 , u32 :: MAX - 1 , u32 :: MAX ,] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; assert ! (! c . is_heap_allocated ()) ; } }
};
}
