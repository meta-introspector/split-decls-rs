// Generated macro for inflateUndermine (function)
macro_rules! DepcrateinflateUndermine {
() => {
// Module: crate
// Provides: {"inflateUndermine"}
// Dependencies: {}
# [doc (hidden)] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`inflateInit_`] or similar"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateUndermine))] pub unsafe extern "C-unwind" fn inflateUndermine (strm : * mut z_stream , subvert : i32) -> c_int { if let Some (stream) = InflateStream :: from_stream_mut (strm) { zlib_rs :: inflate :: undermine (stream , subvert) as i32 } else { ReturnCode :: StreamError as _ } }
};
}
