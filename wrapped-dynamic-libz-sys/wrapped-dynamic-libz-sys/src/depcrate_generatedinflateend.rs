// Generated macro for inflateEnd (function)
macro_rules! Depcrate_generatedinflateEnd {
() => {
// Module: crate::generated
// Provides: {"inflateEnd"}
// Dependencies: {}
pub unsafe fn inflateEnd (strm : z_streamp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "inflateEnd") } , "inflateEnd") . as_bytes ()) . unwrap () ; f (strm) }
};
}
