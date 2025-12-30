// Generated macro for deflateInit_ (function)
macro_rules! DepcratedeflateInit_ {
() => {
// Module: crate
// Provides: {"deflateInit_"}
// Dependencies: {}
# [cfg (zng)] # [inline] pub unsafe fn deflateInit_ (strm : z_streamp , level : c_int , _version : * const c_char , _stream_size : c_int ,) -> c_int { zng_deflateInit (strm , level) }
};
}
