// Generated macro for test_i64_to_compact_string (function)
macro_rules! Depcrate_teststest_i64_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_i64_to_compact_string"}
// Dependencies: {}
# [test] fn test_i64_to_compact_string () { let vals = [i64 :: MIN , i64 :: MIN + 1 , i64 :: MIN + 2 , - 22222222 , - 42 , 0 , 1 , 999 , 123456789 , i64 :: MAX - 2 , i64 :: MAX - 1 , i64 :: MAX ,] ; for x in & vals { let c = x . to_compact_string () ; let s = x . to_string () ; assert_eq ! (c , s) ; # [cfg (target_pointer_width = "64")] assert ! (! c . is_heap_allocated ()) ; } }
};
}
