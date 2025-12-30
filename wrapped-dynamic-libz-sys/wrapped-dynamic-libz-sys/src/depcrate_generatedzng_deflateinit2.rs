// Generated macro for zng_deflateInit2 (function)
macro_rules! Depcrate_generatedzng_deflateInit2 {
() => {
// Module: crate::generated
// Provides: {"zng_deflateInit2"}
// Dependencies: {}
# [cfg (zng)] pub unsafe fn zng_deflateInit2 (strm : z_streamp , level : c_int , method : c_int , windowBits : c_int , memLevel : c_int , strategy : c_int ,) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , level : c_int , method : c_int , windowBits : c_int , memLevel : c_int , strategy : c_int ,) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "zng_deflateInit2") } , "zng_deflateInit2") . as_bytes ()) . unwrap () ; f (strm , level , method , windowBits , memLevel , strategy) }
};
}
