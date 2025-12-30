// Generated macro for test_roundtrip_from_string_empty_string (function)
macro_rules! Depcrate_teststest_roundtrip_from_string_empty_string {
() => {
// Module: crate::tests
// Provides: {"test_roundtrip_from_string_empty_string"}
// Dependencies: {}
# [test] fn test_roundtrip_from_string_empty_string () { let string = String :: new () ; let str_ptr = string . as_ptr () ; let str_len = string . len () ; let str_cap = string . capacity () ; let compact = CompactString :: from (string) ; assert ! (! compact . is_heap_allocated ()) ; let new_string = String :: from (compact) ; let new_str_ptr = new_string . as_ptr () ; let new_str_len = new_string . len () ; let new_str_cap = new_string . capacity () ; assert_eq ! (str_ptr , new_str_ptr) ; assert_eq ! (str_len , new_str_len) ; assert_eq ! (str_cap , new_str_cap) ; }
};
}
