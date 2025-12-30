// Generated macro for gzdopen (function)
macro_rules! Depcrate_gzgzdopen {
() => {
// Module: crate::gz
// Provides: {"gzdopen"}
// Dependencies: {}
# [doc = " Given an open file descriptor, prepare to read or write a gzip file."] # [doc = " NOTE: This is similar to [`gzopen`], but for cases where the caller already"] # [doc = " has the file open."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * If successful, an opaque handle that the caller can later free with [`gzfree`]"] # [doc = " * On error, a null pointer"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `mode` points to a valid C string. If the"] # [doc = " return value is non-NULL, caller must delete it using only [`gzclose`]."] # [doc = ""] # [doc = " [`gzfree`]: crate::z_stream"] # [export_name = crate :: prefix ! (gzdopen)] pub unsafe extern "C-unwind" fn gzdopen (fd : c_int , mode : * const c_char) -> gzFile { unsafe { gzopen_help (Source :: Fd (fd) , mode) } }
};
}
