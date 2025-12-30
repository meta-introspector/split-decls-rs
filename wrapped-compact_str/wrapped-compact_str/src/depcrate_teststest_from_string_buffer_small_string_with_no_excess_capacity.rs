// Generated macro for test_from_string_buffer_small_string_with_no_excess_capacity (function)
macro_rules! Depcrate_teststest_from_string_buffer_small_string_with_no_excess_capacity {
() => {
// Module: crate::tests
// Provides: {"test_from_string_buffer_small_string_with_no_excess_capacity"}
// Dependencies: {}
# [test] fn test_from_string_buffer_small_string_with_no_excess_capacity () { let string = String :: from ("abcdefg") ; let str_ptr = string . as_ptr () ; let str_len = string . len () ; let str_cap = string . capacity () ; let compact = CompactString :: from_string_buffer (string) ; assert ! (compact . is_heap_allocated ()) ; let cpt_ptr = compact . as_ptr () ; let cpt_len = compact . len () ; let cpt_cap = compact . capacity () ; assert_eq ! (str_ptr , cpt_ptr) ; assert_eq ! (str_len , cpt_len) ; assert_eq ! (str_cap , cpt_cap) ; }
};
}
