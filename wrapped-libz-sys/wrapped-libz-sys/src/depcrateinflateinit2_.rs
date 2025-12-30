// Generated macro for inflateInit2_ (function)
macro_rules! DepcrateinflateInit2_ {
() => {
// Module: crate
// Provides: {"inflateInit2_"}
// Dependencies: {}
# [cfg (zng)] # [inline (always)] pub unsafe fn inflateInit2_ (strm : z_streamp , windowBits : c_int , _version : * const c_char , _stream_size : c_int ,) -> c_int { zng_inflateInit2 (strm , windowBits) }
};
}
