// Generated macro for inflateBackInit_ (function)
macro_rules! Depcrate_generatedinflateBackInit_ {
() => {
// Module: crate::generated
// Provides: {"inflateBackInit_"}
// Dependencies: {}
# [cfg (zng)] # [inline (always)] pub unsafe fn inflateBackInit_ (strm : z_streamp , windowBits : c_int , window : * mut c_uchar , _version : * const c_char , _stream_size : c_int ,) -> c_int { zng_inflateBackInit (strm , windowBits , window) }
};
}
