// Generated macro for zng_inflateBackInit (function)
macro_rules! Depcrate_generatedzng_inflateBackInit {
() => {
// Module: crate::generated
// Provides: {"zng_inflateBackInit"}
// Dependencies: {}
# [cfg (zng)] pub unsafe fn zng_inflateBackInit (strm : z_streamp , windowBits : c_int , window : * mut c_uchar ,) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , windowBits : c_int , window : * mut c_uchar) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "zng_inflateBackInit") } , "zng_inflateBackInit") . as_bytes () ,) . unwrap () ; f (strm , windowBits , window) }
};
}
