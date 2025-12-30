// Generated macro for other_32 (other)
macro_rules! Depcrateother_32 {
() => {
// Module: crate
// Provides: {"other_32"}
// Dependencies: {}
# [cfg (not (zng))] extern "C" { pub fn deflateInit_ (strm : z_streamp , level : c_int , version : * const c_char , stream_size : c_int ,) -> c_int ; pub fn deflateInit2_ (strm : z_streamp , level : c_int , method : c_int , windowBits : c_int , memLevel : c_int , strategy : c_int , version : * const c_char , stream_size : c_int ,) -> c_int ; pub fn inflateBackInit_ (strm : z_streamp , windowBits : c_int , window : * mut c_uchar , version : * const c_char , stream_size : c_int ,) -> c_int ; pub fn inflateInit_ (strm : z_streamp , version : * const c_char , stream_size : c_int) -> c_int ; pub fn inflateInit2_ (strm : z_streamp , windowBits : c_int , version : * const c_char , stream_size : c_int ,) -> c_int ; }
};
}
