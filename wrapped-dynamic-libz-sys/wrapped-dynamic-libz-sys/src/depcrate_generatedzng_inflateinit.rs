// Generated macro for zng_inflateInit (function)
macro_rules! Depcrate_generatedzng_inflateInit {
() => {
// Module: crate::generated
// Provides: {"zng_inflateInit"}
// Dependencies: {}
# [cfg (zng)] pub unsafe fn zng_inflateInit (strm : z_streamp) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "zng_inflateInit") } , "zng_inflateInit") . as_bytes ()) . unwrap () ; f (strm) }
};
}
