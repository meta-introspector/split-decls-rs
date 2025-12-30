// Generated macro for impl_38 (impl)
macro_rules! Depcrate_to_fmtimpl_38 {
() => {
// Module: crate::to_fmt
// Provides: {"impl_38"}
// Dependencies: {}
impl NumberTextHandler { fn text_fragment (& mut self , v : & str , mut out : impl Write) -> fmt :: Result { if ! self . is_nan_or_infinity { let mut range = 0 .. 0 ; for b in v . as_bytes () { match b { b'0' if self . at_start => { self . leading_zeroes += 1 ; range . start += 1 ; range . end += 1 ; } b'0' ..= b'9' => { if self . at_start && self . sign_negative { _try_no_conv ! (out . write_char ('-')) ; } self . at_start = false ; range . end += 1 ; } b'.' => { if self . at_start { if self . sign_negative { _try_no_conv ! (out . write_char ('-')) ; } _try_no_conv ! (out . write_char ('0')) ; } self . at_start = false ; range . end += 1 ; } b'-' if self . at_start => { self . sign_negative = true ; range . start += 1 ; range . end += 1 ; } b'+' if self . at_start => { range . start += 1 ; range . end += 1 ; } b's' | b'n' | b'i' | b'S' | b'N' | b'I' => { self . is_nan_or_infinity = true ; self . at_start = false ; _try_no_conv ! (out . write_str ("null")) ; range . start = 0 ; range . end = 0 ; break ; } _ => range . end += 1 , } } _try_no_conv ! (out . write_str (& v [range])) ; } Ok (()) } fn end (& mut self , mut out : impl Write) -> fmt :: Result { if self . at_start { _try_no_conv ! (out . write_char ('0')) ; } Ok (()) } }
};
}
