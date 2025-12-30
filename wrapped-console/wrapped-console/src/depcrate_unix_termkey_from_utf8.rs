// Generated macro for key_from_utf8 (function)
macro_rules! Depcrate_unix_termkey_from_utf8 {
() => {
// Module: crate::unix_term
// Provides: {"key_from_utf8"}
// Dependencies: {}
fn key_from_utf8 (buf : & [u8]) -> Key { if let Ok (s) = str :: from_utf8 (buf) { if let Some (c) = s . chars () . next () { return Key :: Char (c) ; } } Key :: Unknown }
};
}
