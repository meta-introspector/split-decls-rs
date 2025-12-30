// Generated macro for gzgetc_ (function)
macro_rules! Depcrate_gzgzgetc_ {
() => {
// Module: crate::gz
// Provides: {"gzgetc_"}
// Dependencies: {}
# [doc = " Backward-compatibility alias for [`gzgetc`]."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - The byte read, on success."] # [doc = " - `-1` on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzgetc_)] pub unsafe extern "C-unwind" fn gzgetc_ (file : gzFile) -> c_int { unsafe { gzgetc (file) } }
};
}
