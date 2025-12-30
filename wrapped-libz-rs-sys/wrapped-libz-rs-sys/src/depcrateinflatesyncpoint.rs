// Generated macro for inflateSyncPoint (function)
macro_rules! DepcrateinflateSyncPoint {
() => {
// Module: crate
// Provides: {"inflateSyncPoint"}
// Dependencies: {}
# [doc (hidden)] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`inflateInit_`] or similar"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateSyncPoint))] pub unsafe extern "C-unwind" fn inflateSyncPoint (strm : * mut z_stream) -> i32 { if let Some (stream) = InflateStream :: from_stream_mut (strm) { zlib_rs :: inflate :: sync_point (stream) as i32 } else { ReturnCode :: StreamError as _ } }
};
}
