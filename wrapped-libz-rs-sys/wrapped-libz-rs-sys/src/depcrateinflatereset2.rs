// Generated macro for inflateReset2 (function)
macro_rules! DepcrateinflateReset2 {
() => {
// Module: crate
// Provides: {"inflateReset2"}
// Dependencies: {}
# [doc = " This function is the same as [`inflateReset`], but it also permits changing the wrap and window size requests."] # [doc = ""] # [doc = " The `windowBits` parameter is interpreted the same as it is for [`inflateInit2_`]."] # [doc = " If the window size is changed, then the memory allocated for the window is freed, and the window will be reallocated by [`inflate`] if needed."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - [`Z_OK`] if success"] # [doc = " - [`Z_STREAM_ERROR`] if the source stream state was inconsistent, or if the `windowBits`"] # [doc = "   parameter is invalid"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`inflateInit_`] or similar"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateReset2))] pub unsafe extern "C-unwind" fn inflateReset2 (strm : * mut z_stream , windowBits : c_int) -> i32 { if let Some (stream) = InflateStream :: from_stream_mut (strm) { let config = InflateConfig { window_bits : windowBits , } ; zlib_rs :: inflate :: reset_with_config (stream , config) as _ } else { ReturnCode :: StreamError as _ } }
};
}
