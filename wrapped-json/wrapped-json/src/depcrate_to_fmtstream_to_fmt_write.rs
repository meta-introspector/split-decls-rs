// Generated macro for stream_to_fmt_write (function)
macro_rules! Depcrate_to_fmtstream_to_fmt_write {
() => {
// Module: crate::to_fmt
// Provides: {"stream_to_fmt_write"}
// Dependencies: {}
# [doc = "\nStream a value as JSON to an underlying formatter.\n"] pub fn stream_to_fmt_write (fmt : impl Write , v : impl sval :: Value) -> Result < () , Error > { let mut stream = Formatter :: new (fmt) ; match v . stream (& mut stream) { Ok (()) => Ok (()) , Err (_) => Err (stream . err . unwrap_or_else (Error :: generic)) , } }
};
}
