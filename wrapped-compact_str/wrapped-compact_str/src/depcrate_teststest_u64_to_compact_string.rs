// Generated macro for test_u64_to_compact_string (function)
macro_rules! Depcrate_teststest_u64_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_u64_to_compact_string"}
// Dependencies: {}
# [test] fn test_u64_to_compact_string () { let vals = [u64 :: MIN , 1 , 999 , 123456789 , 98765432123456 , u64 :: MAX - 2 , u64 :: MAX - 1 , u64 :: MAX ,] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; # [cfg (target_pointer_width = "64")] assert ! (! c . is_heap_allocated ()) ; } }
};
}
