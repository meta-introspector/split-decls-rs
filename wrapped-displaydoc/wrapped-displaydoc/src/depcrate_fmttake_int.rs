// Generated macro for take_int (function)
macro_rules! Depcrate_fmttake_int {
() => {
// Module: crate::fmt
// Provides: {"take_int"}
// Dependencies: {}
fn take_int (read : & mut & str) -> String { let mut int = String :: new () ; int . push ('_') ; for (i , ch) in read . char_indices () { match ch { '0' ..= '9' => int . push (ch) , _ => { * read = & read [i ..] ; break ; } } } int }
};
}
