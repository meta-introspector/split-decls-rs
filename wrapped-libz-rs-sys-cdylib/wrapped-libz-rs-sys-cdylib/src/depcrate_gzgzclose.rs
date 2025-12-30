// Generated macro for gzclose (function)
macro_rules! Depcrate_gzgzclose {
() => {
// Module: crate::gz
// Provides: {"gzclose"}
// Dependencies: {}
# [doc = " Close an open gzip file and free the internal data structures referenced by the file handle."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * [`Z_ERRNO`] if closing the file failed"] # [doc = " * [`Z_OK`] otherwise"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `file` must be one of the following:"] # [doc = " - A file handle must have been obtained from a function in this library, such as [`gzopen`]."] # [doc = " - A null pointer."] # [doc = ""] # [doc = " This function may be called at most once for any file handle."] # [doc = ""] # [doc = " `file` must not be used after this call returns, as the memory it references may have"] # [doc = " been deallocated."] # [export_name = crate :: prefix ! (gzclose)] pub unsafe extern "C-unwind" fn gzclose (file : gzFile) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_ref () }) else { return Z_STREAM_ERROR ; } ; match state . mode { GzMode :: GZ_READ => unsafe { gzclose_r (file) } , GzMode :: GZ_WRITE | GzMode :: GZ_APPEND | GzMode :: GZ_NONE => unsafe { gzclose_w (file) } , } }
};
}
