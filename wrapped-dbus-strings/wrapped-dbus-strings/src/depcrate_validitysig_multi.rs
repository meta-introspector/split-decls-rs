// Generated macro for sig_multi (function)
macro_rules! Depcrate_validitysig_multi {
() => {
// Module: crate::validity
// Provides: {"sig_multi"}
// Dependencies: {}
fn sig_multi (s : & [u8] , arrs : u8 , structs : u8) -> Option < usize > { let mut pos = 0 ; while pos < s . len () { if s . get (pos) == Some (& b')') { return Some (pos) } pos += sig_single (& s [pos ..] , arrs , structs) ? ; } Some (pos) }
};
}
