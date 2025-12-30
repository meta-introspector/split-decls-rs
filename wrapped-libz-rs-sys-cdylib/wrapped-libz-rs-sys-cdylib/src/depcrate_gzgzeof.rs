// Generated macro for gzeof (function)
macro_rules! Depcrate_gzgzeof {
() => {
// Module: crate::gz
// Provides: {"gzeof"}
// Dependencies: {}
# [doc = " Check whether a read operation has tried to read beyond the end of `file`."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * 1 if the end-of-file indicator is set. Note that this indicator is set only"] # [doc = "   if a read tries to go past the end of the input. If the last read request"] # [doc = "   attempted to read exactly the number of bytes remaining in the file, the"] # [doc = "   end-of-file indicator will not be set."] # [doc = " * 0 the end-of-file indicator is not set or `file` is null"] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `file` - A gzip file handle, or null"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzeof)] pub unsafe extern "C-unwind" fn gzeof (file : gzFile) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_ref () }) else { return 0 ; } ; if state . mode != GzMode :: GZ_READ { return 0 ; } state . past as _ }
};
}
