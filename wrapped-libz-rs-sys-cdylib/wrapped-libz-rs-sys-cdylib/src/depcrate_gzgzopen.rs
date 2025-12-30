// Generated macro for gzopen (function)
macro_rules! Depcrate_gzgzopen {
() => {
// Module: crate::gz
// Provides: {"gzopen"}
// Dependencies: {}
# [doc = " Open a gzip file for reading or writing."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * If successful, an opaque handle that the caller can later free with [`gzfree`]"] # [doc = " * On error, a null pointer"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `path` and `mode` point to valid C strings. If the"] # [doc = " return value is non-NULL, caller must delete it using only [`gzclose`]."] # [doc = ""] # [doc = " [`gzfree`]: crate::z_stream"] # [export_name = crate :: prefix ! (gzopen)] pub unsafe extern "C-unwind" fn gzopen (path : * const c_char , mode : * const c_char) -> gzFile { if path . is_null () { return ptr :: null_mut () ; } let source = Source :: Path (path) ; unsafe { gzopen_help (source , mode) } }
};
}
