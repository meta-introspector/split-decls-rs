// Generated macro for test_from_string_buffer_inlines_on_clone (function)
macro_rules! Depcrate_teststest_from_string_buffer_inlines_on_clone {
() => {
// Module: crate::tests
// Provides: {"test_from_string_buffer_inlines_on_clone"}
// Dependencies: {}
# [test] fn test_from_string_buffer_inlines_on_clone () { let a = CompactString :: from_string_buffer ("hello" . to_string ()) ; assert ! (a . is_heap_allocated ()) ; let b = a . clone () ; assert ! (! b . is_heap_allocated ()) ; }
};
}
