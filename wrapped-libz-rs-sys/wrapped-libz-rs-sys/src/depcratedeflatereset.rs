// Generated macro for deflateReset (function)
macro_rules! DepcratedeflateReset {
() => {
// Module: crate
// Provides: {"deflateReset"}
// Dependencies: {}
# [doc = " This function is equivalent to [`deflateEnd`] followed by [`deflateInit_`], but does not free and reallocate the internal compression state."] # [doc = ""] # [doc = " This function will leave the compression level and any other attributes that may have been set unchanged."] # [doc = " The stream's `total_in`, `total_out`, `adler`, and `msg` fields are initialized."] # [doc = ""] # [doc = " ## Returns"] # [doc = ""] # [doc = " - [`Z_OK`] if success"] # [doc = " - [`Z_STREAM_ERROR`] if the stream state was inconsistent"] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`deflateInit_`] or similar"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (deflateReset))] pub unsafe extern "C-unwind" fn deflateReset (strm : * mut z_stream) -> i32 { match DeflateStream :: from_stream_mut (strm) { Some (stream) => zlib_rs :: deflate :: reset (stream) as _ , None => ReturnCode :: StreamError as _ , } }
};
}
