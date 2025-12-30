// Generated macro for gzflush (function)
macro_rules! Depcrate_gzgzflush {
() => {
// Module: crate::gz
// Provides: {"gzflush"}
// Dependencies: {}
# [doc = " Flush all pending output buffered in `file`. The parameter `flush` is interpreted"] # [doc = " the same way as in the [`deflate`] function. The return value is the zlib error"] # [doc = " number (see [`gzerror`]). `gzflush` is permitted only when writing."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - `Z_OK` on success."] # [doc = " - a `Z_` error code on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzflush)] pub unsafe extern "C-unwind" fn gzflush (file : gzFile , flush : c_int) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return Z_STREAM_ERROR ; } ; if state . mode != GzMode :: GZ_WRITE || state . err != Z_OK { return Z_STREAM_ERROR ; } if ! (0 ..= Z_FINISH) . contains (& flush) { return Z_STREAM_ERROR ; } if state . seek { state . seek = false ; if gz_zero (state , state . skip as _) . is_err () { return state . err ; } } let _ = gz_comp (state , flush) ; state . err }
};
}
