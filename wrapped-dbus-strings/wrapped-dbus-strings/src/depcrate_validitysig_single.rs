// Generated macro for sig_single (function)
macro_rules! Depcrate_validitysig_single {
() => {
// Module: crate::validity
// Provides: {"sig_single"}
// Dependencies: {}
pub (crate) fn sig_single (s : & [u8] , arrs : u8 , structs : u8) -> Option < usize > { s . first () . and_then (| c | { if BASIC_TYPES . into_iter () . any (| x | x == c) { Some (1) } else { Some (1 + match c { b'v' => 0 , b'a' => { if arrs >= 32 { None ? } ; if s . get (1) == Some (& b'{') { let c = s . get (2) ? ; if ! BASIC_TYPES . into_iter () . any (| x | x == c) { None ? } ; let pos = 3 + sig_single (& s [3 ..] , arrs + 1 , structs) ? ; if s . get (pos) ? != & b'}' { None ? } pos } else { sig_single (& s [1 ..] , arrs + 1 , structs) ? } } , b'(' => { if structs >= 32 { None ? } ; let pos = 1 + sig_multi (& s [1 ..] , arrs , structs + 1) ? ; if pos == 1 || s . get (pos) ? != & b')' { None ? } pos } , _ => None ? , }) } }) }
};
}
