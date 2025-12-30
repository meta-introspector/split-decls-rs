// Generated macro for write_char_to_vec (function)
macro_rules! Depcrate_functionswrite_char_to_vec {
() => {
// Module: crate::functions
// Provides: {"write_char_to_vec"}
// Dependencies: {}
# [inline] pub (crate) fn write_char_to_vec (c : char , output : & mut Vec < u8 >) { let width = c . len_utf8 () ; output . reserve (width) ; let current_length = output . len () ; unsafe { output . set_len (current_length + width) ; } c . encode_utf8 (& mut output [current_length ..]) ; }
};
}
