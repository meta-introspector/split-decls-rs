// Generated macro for inflateInit2 (function)
macro_rules! DepcrateinflateInit2 {
() => {
// Module: crate
// Provides: {"inflateInit2"}
// Dependencies: {}
# [doc = " Helper that implements the actual initialization logic"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm`"] # [doc = " * If `strm` is not `NULL`, the following fields contain valid values"] # [doc = "     - `zalloc`"] # [doc = "     - `zfree`"] # [doc = "     - `opaque`"] unsafe extern "C-unwind" fn inflateInit2 (strm : z_streamp , windowBits : c_int) -> c_int { let Some (strm) = (unsafe { strm . as_mut () }) else { return ReturnCode :: StreamError as _ ; } ; let config = InflateConfig { window_bits : windowBits , } ; zlib_rs :: inflate :: init (strm , config) as _ }
};
}
