// Generated macro for is_ascii_no_nul (function)
macro_rules! Depcrate___ns_macro_helpers_ns_stringis_ascii_no_nul {
() => {
// Module: crate::__ns_macro_helpers::ns_string
// Provides: {"is_ascii_no_nul"}
// Dependencies: {}
# [doc = " Returns `true` if `bytes` is entirely ASCII with no interior NULs."] pub const fn is_ascii_no_nul (bytes : & [u8]) -> bool { let mut i = 0 ; while i < bytes . len () { let byte = bytes [i] ; if ! byte . is_ascii () || byte == b'\0' { return false ; } i += 1 ; } true }
};
}
