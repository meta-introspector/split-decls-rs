// Generated macro for test_into_string_empty_str (function)
macro_rules! Depcrate_teststest_into_string_empty_str {
() => {
// Module: crate::tests
// Provides: {"test_into_string_empty_str"}
// Dependencies: {}
# [test] fn test_into_string_empty_str () { let data = "" ; let str_len = data . len () ; let compact = CompactString :: from (data) ; let new_string = String :: from (compact) ; let new_str_addr = new_string . as_ptr () ; let new_str_len = new_string . len () ; let new_str_cap = new_string . capacity () ; let empty_string = String :: new () ; assert_eq ! (empty_string . as_ptr () , new_str_addr) ; assert_eq ! (str_len , new_str_len) ; assert_eq ! (str_len , new_str_cap) ; }
};
}
