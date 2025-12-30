// Generated macro for test_bool_to_compact_string (function)
macro_rules! Depcrate_teststest_bool_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"test_bool_to_compact_string"}
// Dependencies: {}
# [test] fn test_bool_to_compact_string () { let c = true . to_compact_string () ; let s = true . to_string () ; assert_eq ! ("true" , c) ; assert_eq ! (c , s) ; assert ! (! c . is_heap_allocated ()) ; let c = false . to_compact_string () ; let s = false . to_string () ; assert_eq ! ("false" , c) ; assert_eq ! (c , s) ; assert ! (! c . is_heap_allocated ()) ; }
};
}
