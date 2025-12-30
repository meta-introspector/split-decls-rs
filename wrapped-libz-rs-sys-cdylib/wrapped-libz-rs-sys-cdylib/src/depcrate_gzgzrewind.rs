// Generated macro for gzrewind (function)
macro_rules! Depcrate_gzgzrewind {
() => {
// Module: crate::gz
// Provides: {"gzrewind"}
// Dependencies: {}
# [doc = " Rewind `file` to the start. This function is supported only for reading."] # [doc = ""] # [doc = " Note: `gzrewind(file)` is equivalent to [`gzseek`]`(file, 0, SEEK_SET)`"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - `0` on success."] # [doc = " - `-1` on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzrewind)] pub unsafe extern "C-unwind" fn gzrewind (file : gzFile) -> c_int { let Some (state) = (unsafe { file . cast :: < GzState > () . as_mut () }) else { return - 1 ; } ; if state . mode != GzMode :: GZ_READ || (state . err != Z_OK && state . err != Z_BUF_ERROR) { return - 1 ; } if lseek64 (state . fd , state . start as _ , SEEK_SET) == - 1 { return - 1 ; } gz_reset (state) ; 0 }
};
}
