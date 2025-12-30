// Generated macro for inflateBackEnd (function)
macro_rules! DepcrateinflateBackEnd {
() => {
// Module: crate
// Provides: {"inflateBackEnd"}
// Dependencies: {}
# [doc = " Deallocates all dynamically allocated data structures for this stream."] # [doc = ""] # [doc = " This function discards any unprocessed input and does not flush any pending output."] # [doc = ""] # [doc = " ## Returns"] # [doc = ""] # [doc = " - [`Z_OK`] if success"] # [doc = " - [`Z_STREAM_ERROR`] if the stream state was inconsistent"] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`inflateBackInit_`]"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateBackEnd))] pub unsafe extern "C-unwind" fn inflateBackEnd (_strm : z_streamp) -> c_int { todo ! ("inflateBack is not implemented yet") }
};
}
