// Generated macro for fmt (function)
macro_rules! Depcrate_util_fmtfmt {
() => {
// Module: crate::util::fmt
// Provides: {"fmt"}
// Dependencies: {}
pub (crate) fn fmt < T : Display > (fmt : T) -> HeaderValue { let s = fmt . to_string () ; match HeaderValue :: from_maybe_shared (s) { Ok (val) => val , Err (err) => panic ! ("illegal HeaderValue; error = {:?}, fmt = \"{}\"" , err , fmt) , } }
};
}
