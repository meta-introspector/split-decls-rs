// Generated macro for zng_inflateInit2 (function)
macro_rules! Depcrate_generatedzng_inflateInit2 {
() => {
// Module: crate::generated
// Provides: {"zng_inflateInit2"}
// Dependencies: {}
# [cfg (zng)] pub unsafe fn zng_inflateInit2 (strm : z_streamp , windowBits : c_int) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , windowBits : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "zng_inflateInit2") } , "zng_inflateInit2") . as_bytes ()) . unwrap () ; f (strm , windowBits) }
};
}
