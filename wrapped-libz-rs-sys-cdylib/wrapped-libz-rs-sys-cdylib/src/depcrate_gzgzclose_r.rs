// Generated macro for gzclose_r (function)
macro_rules! Depcrate_gzgzclose_r {
() => {
// Module: crate::gz
// Provides: {"gzclose_r"}
// Dependencies: {}
# [doc = " Close a gzip file that was opened for reading."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * Z_OK if `state` has no outstanding error and the file is closed successfully."] # [doc = " * A Z_ error code if the `state` is null or the file close operation fails."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `file` must be one of the following:"] # [doc = " - A file handle must have been obtained from a function in this library, such as [`gzopen`]."] # [doc = " - A null pointer."] # [doc = ""] # [doc = " `file` must not be used after this call returns, as the memory it references may have"] # [doc = " been deallocated."] # [export_name = crate :: prefix ! (gzclose_r)] pub unsafe extern "C-unwind" fn gzclose_r (file : gzFile) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return Z_STREAM_ERROR ; } ; if state . mode != GzMode :: GZ_READ { return Z_STREAM_ERROR ; } if state . in_size != 0 { unsafe { inflateEnd (& mut state . stream as * mut z_stream) } ; } let err = match state . err { Z_BUF_ERROR => Z_BUF_ERROR , _ => Z_OK , } ; let ret = match unsafe { libc :: close (state . fd) } { 0 => err , _ => Z_ERRNO , } ; unsafe { free_state (file . cast :: < GzState > ()) } ; ret }
};
}
