// Generated macro for verify_name (function)
macro_rules! Depcrate_parseverify_name {
() => {
// Module: crate::parse
// Provides: {"verify_name"}
// Dependencies: {}
# [doc = " Check whether a struct or union name is a valid identifier"] pub (crate) const fn verify_name (name : & str) -> bool { let bytes = name . as_bytes () ; if let b"?" = bytes { return true ; } if bytes . is_empty () { return false ; } let mut i = 0 ; while i < bytes . len () { let byte = bytes [i] ; if ! (byte . is_ascii_alphanumeric () || byte == b'_') { return false ; } i += 1 ; } true }
};
}
