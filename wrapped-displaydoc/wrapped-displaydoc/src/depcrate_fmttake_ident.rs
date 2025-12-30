// Generated macro for take_ident (function)
macro_rules! Depcrate_fmttake_ident {
() => {
// Module: crate::fmt
// Provides: {"take_ident"}
// Dependencies: {}
fn take_ident (read : & mut & str) -> String { let mut ident = String :: new () ; for (i , ch) in read . char_indices () { match ch { 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => ident . push (ch) , _ => { * read = & read [i ..] ; break ; } } } ident }
};
}
