// Generated macro for gzdirect (function)
macro_rules! Depcrate_gzgzdirect {
() => {
// Module: crate::gz
// Provides: {"gzdirect"}
// Dependencies: {}
# [doc = " Check whether `file` is in direct mode (reading or writing literal bytes without compression)."] # [doc = ""] # [doc = " NOTE: If `gzdirect` is called immediately after [`gzopen`] or [`gzdopen`], it may allocate"] # [doc = " buffers internally to read the file header and determine whether the content is a gzip file."] # [doc = " If [`gzbuffer`] is used, it should be called before `gzdirect`."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " 0 if `file` is null."] # [doc = ""] # [doc = " If `file` is being read,"] # [doc = " * 1 if the contents are being read directly, without decompression."] # [doc = " * 0 if the contents are being decompressed when read."] # [doc = ""] # [doc = " If `file` is being written,"] # [doc = " * 1 if transparent mode was requested upon open (with the `\"wT\"` mode flag for [`gzopen`])."] # [doc = " * 0 otherwise."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `file` - A gzip file handle, or null"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzdirect)] pub unsafe extern "C-unwind" fn gzdirect (file : gzFile) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return 0 ; } ; if state . mode == GzMode :: GZ_READ && state . how == How :: Look && state . have == 0 { let _ = unsafe { gz_look (state) } ; } state . direct as _ }
};
}
