// Generated macro for other_33 (other)
macro_rules! Depcrateother_33 {
() => {
// Module: crate
// Provides: {"other_33"}
// Dependencies: {}
# [cfg (zng)] extern "C" { pub fn zng_deflateInit (strm : z_streamp , level : c_int) -> c_int ; pub fn zng_deflateInit2 (strm : z_streamp , level : c_int , method : c_int , windowBits : c_int , memLevel : c_int , strategy : c_int ,) -> c_int ; pub fn zng_inflateBackInit (strm : z_streamp , windowBits : c_int , window : * mut c_uchar) -> c_int ; pub fn zng_inflateInit (strm : z_streamp) -> c_int ; pub fn zng_inflateInit2 (strm : z_streamp , windowBits : c_int) -> c_int ; }
};
}
