// Generated macro for test_into_string_where_32_bit_capacity_is_on_heap (function)
macro_rules! Depcrate_teststest_into_string_where_32_bit_capacity_is_on_heap {
() => {
// Module: crate::tests
// Provides: {"test_into_string_where_32_bit_capacity_is_on_heap"}
// Dependencies: {}
# [test] fn test_into_string_where_32_bit_capacity_is_on_heap () { let buf = vec ! [b'a' ; SIXTEEN_MB - 1] ; let string = unsafe { String :: from_utf8_unchecked (buf) } ; let str_addr = string . as_ptr () ; let str_len = string . len () ; let str_cap = string . capacity () ; let compact = CompactString :: from (string) ; let new_string = String :: from (compact) ; let new_str_addr = new_string . as_ptr () ; let new_str_len = new_string . len () ; let new_str_cap = new_string . capacity () ; assert_eq ! (str_len , new_str_len) ; if cfg ! (target_pointer_width = "64") { assert_eq ! (str_addr , new_str_addr) ; assert_eq ! (str_cap , new_str_cap) ; } else { assert_eq ! (& new_string . as_bytes () [0 .. 10] , b"aaaaaaaaaa") ; assert_eq ! (str_len , new_str_cap) ; } }
};
}
