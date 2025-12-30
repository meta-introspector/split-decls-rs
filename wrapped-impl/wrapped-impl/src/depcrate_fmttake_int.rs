// Generated macro for take_int (function)
macro_rules! Depcrate_fmttake_int {
() => {
// Module: crate::fmt
// Provides: {"take_int"}
// Dependencies: {}
fn take_int < 'a > (read : & mut & 'a str) -> & 'a str { let mut int_len = 0 ; for ch in read . chars () { match ch { '0' ..= '9' => int_len += 1 , _ => break , } } let (int , rest) = read . split_at (int_len) ; * read = rest ; int }
};
}
