// Generated macro for gztell (function)
macro_rules! Depcrate_gzgztell {
() => {
// Module: crate::gz
// Provides: {"gztell"}
// Dependencies: {}
# [doc = " Return the starting position for the next [`gzread`] or [`gzwrite`] on `file`."] # [doc = " This position represents a number of bytes in the uncompressed data stream,"] # [doc = " and is zero when starting, even if appending or reading a gzip stream from"] # [doc = " the middle of a file using [`gzdopen`]."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * The number of bytes prior to the current read or write position in the"] # [doc = "   uncompressed data stream, on success."] # [doc = " * -1 on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gztell)] pub unsafe extern "C-unwind" fn gztell (file : gzFile) -> z_off_t { z_off_t :: try_from (unsafe { gztell64 (file) }) . unwrap_or (- 1) }
};
}
