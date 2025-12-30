// Generated macro for test_into_string_large_string_with_excess_capacity (function)
macro_rules! Depcrate_teststest_into_string_large_string_with_excess_capacity {
() => {
// Module: crate::tests
// Provides: {"test_into_string_large_string_with_excess_capacity"}
// Dependencies: {}
# [test] fn test_into_string_large_string_with_excess_capacity () { let mut string = String :: with_capacity (128) ; string . push_str ("abcdefghijklmnopqrstuvwxyz") ; let str_addr = string . as_ptr () ; let str_len = string . len () ; let str_cap = string . capacity () ; let compact = CompactString :: from (string) ; let new_string = String :: from (compact) ; let new_str_addr = new_string . as_ptr () ; let new_str_len = new_string . len () ; let new_str_cap = new_string . capacity () ; assert_eq ! (str_addr , new_str_addr) ; assert_eq ! (str_len , new_str_len) ; assert_eq ! (str_cap , new_str_cap) ; }
};
}
