// Generated macro for inflateReset (function)
macro_rules! Depcrate_generatedinflateReset {
() => {
// Module: crate::generated
// Provides: {"inflateReset"}
// Dependencies: {}
pub unsafe fn inflateReset (strm : z_streamp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateReset") } , "inflateReset") . as_bytes ()) . unwrap () ; f (strm) }
};
}
