// Generated macro for zng_deflateInit (function)
macro_rules! Depcrate_generatedzng_deflateInit {
() => {
// Module: crate::generated
// Provides: {"zng_deflateInit"}
// Dependencies: {}
# [cfg (zng)] pub unsafe fn zng_deflateInit (strm : z_streamp , level : c_int) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , level : c_int) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "zng_deflateInit") } , "zng_deflateInit") . as_bytes ()) . unwrap () ; f (strm , level) }
};
}
