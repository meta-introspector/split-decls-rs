// Generated macro for test_into_string_small_string_with_excess_capacity (function)
macro_rules! Depcrate_teststest_into_string_small_string_with_excess_capacity {
() => {
// Module: crate::tests
// Provides: {"test_into_string_small_string_with_excess_capacity"}
// Dependencies: {}
# [test] fn test_into_string_small_string_with_excess_capacity () { let mut string = String :: with_capacity (128) ; string . push_str ("abcdef") ; let str_len = string . len () ; let compact = CompactString :: from (string) ; assert ! (! compact . is_heap_allocated ()) ; assert_eq ! (compact . len () , str_len) ; assert_eq ! (compact . capacity () , MAX_SIZE) ; }
};
}
