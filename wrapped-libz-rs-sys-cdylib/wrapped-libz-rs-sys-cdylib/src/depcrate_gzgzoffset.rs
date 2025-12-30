// Generated macro for gzoffset (function)
macro_rules! Depcrate_gzgzoffset {
() => {
// Module: crate::gz
// Provides: {"gzoffset"}
// Dependencies: {}
# [doc = " Return the current compressed (actual) read or write offset of `file`.  This"] # [doc = " offset includes the count of bytes that precede the gzip stream, for example"] # [doc = " when appending or when using [`gzdopen`] for reading. When reading, the"] # [doc = " offset does not include as yet unused buffered input. This information can"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * The number of bytes prior to the current read or write position in the"] # [doc = "   compressed data stream, on success."] # [doc = " * -1 on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzoffset)] pub unsafe extern "C-unwind" fn gzoffset (file : gzFile) -> z_off_t { z_off_t :: try_from (unsafe { gzoffset64 (file) }) . unwrap_or (- 1) }
};
}
