// Generated macro for test_into_string_small_static_str (function)
macro_rules! Depcrate_teststest_into_string_small_static_str {
() => {
// Module: crate::tests
// Provides: {"test_into_string_small_static_str"}
// Dependencies: {}
# [test] fn test_into_string_small_static_str () { let data = "abcdef" ; let str_addr = data . as_ptr () ; let str_len = data . len () ; let compact = CompactString :: const_new (data) ; let new_string = String :: from (compact) ; let new_str_addr = new_string . as_ptr () ; let new_str_len = new_string . len () ; let new_str_cap = new_string . capacity () ; assert_ne ! (str_addr , new_str_addr) ; assert_eq ! (str_len , new_str_len) ; assert_eq ! (str_len , new_str_cap) ; }
};
}
