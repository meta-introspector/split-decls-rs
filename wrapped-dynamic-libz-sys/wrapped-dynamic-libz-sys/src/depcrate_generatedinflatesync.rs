// Generated macro for inflateSync (function)
macro_rules! Depcrate_generatedinflateSync {
() => {
// Module: crate::generated
// Provides: {"inflateSync"}
// Dependencies: {}
pub unsafe fn inflateSync (strm : z_streamp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateSync") } , "inflateSync") . as_bytes ()) . unwrap () ; f (strm) }
};
}
