// Generated macro for deflateInit2_ (function)
macro_rules! DepcratedeflateInit2_ {
() => {
// Module: crate
// Provides: {"deflateInit2_"}
// Dependencies: {}
# [cfg (zng)] # [inline (always)] pub unsafe fn deflateInit2_ (strm : z_streamp , level : c_int , method : c_int , windowBits : c_int , memLevel : c_int , strategy : c_int , _version : * const c_char , _stream_size : c_int ,) -> c_int { zng_deflateInit2 (strm , level , method , windowBits , memLevel , strategy) }
};
}
