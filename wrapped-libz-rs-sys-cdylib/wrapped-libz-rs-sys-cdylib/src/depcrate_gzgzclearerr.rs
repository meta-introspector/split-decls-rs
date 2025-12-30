// Generated macro for gzclearerr (function)
macro_rules! Depcrate_gzgzclearerr {
() => {
// Module: crate::gz
// Provides: {"gzclearerr"}
// Dependencies: {}
# [doc = " Clear the error and end-of-file state for `file`."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `file` - A gzip file handle, or null"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzclearerr)] pub unsafe extern "C-unwind" fn gzclearerr (file : gzFile) { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return ; } ; if state . mode != GzMode :: GZ_READ && state . mode != GzMode :: GZ_WRITE { return ; } if state . mode == GzMode :: GZ_READ { state . eof = false ; state . past = false ; } unsafe { gz_error (state , None) } ; }
};
}
