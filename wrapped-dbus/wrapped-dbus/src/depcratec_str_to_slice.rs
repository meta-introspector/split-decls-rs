// Generated macro for c_str_to_slice (function)
macro_rules! Depcratec_str_to_slice {
() => {
// Module: crate
// Provides: {"c_str_to_slice"}
// Dependencies: {}
fn c_str_to_slice (c : & * const c_char) -> Option < & str > { if c . is_null () { None } else { std :: str :: from_utf8 (unsafe { CStr :: from_ptr (* c) . to_bytes () }) . ok () } }
};
}
