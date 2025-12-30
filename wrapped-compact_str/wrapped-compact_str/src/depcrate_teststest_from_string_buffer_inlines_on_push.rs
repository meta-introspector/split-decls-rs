// Generated macro for test_from_string_buffer_inlines_on_push (function)
macro_rules! Depcrate_teststest_from_string_buffer_inlines_on_push {
() => {
// Module: crate::tests
// Provides: {"test_from_string_buffer_inlines_on_push"}
// Dependencies: {}
# [test] fn test_from_string_buffer_inlines_on_push () { let mut compact = CompactString :: from_string_buffer ("hello" . to_string ()) ; assert ! (compact . is_heap_allocated ()) ; compact . push_str (" world") ; assert ! (! compact . is_heap_allocated ()) ; }
};
}
