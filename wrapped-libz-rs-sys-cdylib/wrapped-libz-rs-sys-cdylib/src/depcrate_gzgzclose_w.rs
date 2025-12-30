// Generated macro for gzclose_w (function)
macro_rules! Depcrate_gzgzclose_w {
() => {
// Module: crate::gz
// Provides: {"gzclose_w"}
// Dependencies: {}
# [doc = " Close a gzip file that was opened for writing."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * Z_OK if `state` has no outstanding error and the file is closed successfully."] # [doc = " * A Z_ error code if the `state` is null or the file close operation fails."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `file` must be one of the following:"] # [doc = " - A file handle must have been obtained from a function in this library, such as [`gzopen`]."] # [doc = " - A null pointer."] # [doc = ""] # [doc = " `file` must not be used after this call returns, as the memory it references may have"] # [doc = " been deallocated."] # [export_name = crate :: prefix ! (gzclose_w)] pub unsafe extern "C-unwind" fn gzclose_w (file : gzFile) -> c_int { let mut ret = Z_OK ; let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return Z_STREAM_ERROR ; } ; if state . mode != GzMode :: GZ_WRITE { return Z_STREAM_ERROR ; } if state . seek { state . seek = false ; if gz_zero (state , state . skip as _) . is_err () { ret = state . err ; } } if gz_comp (state , Z_FINISH) . is_err () { ret = state . err ; } if state . in_size != 0 && ! state . direct { unsafe { deflateEnd (& mut state . stream as * mut z_stream) } ; } if unsafe { libc :: close (state . fd) } == - 1 { ret = Z_ERRNO ; } unsafe { free_state (file . cast :: < GzState > ()) } ; ret }
};
}
