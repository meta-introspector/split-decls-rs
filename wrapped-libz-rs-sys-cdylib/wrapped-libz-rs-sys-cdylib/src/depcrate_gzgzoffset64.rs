// Generated macro for gzoffset64 (function)
macro_rules! Depcrate_gzgzoffset64 {
() => {
// Module: crate::gz
// Provides: {"gzoffset64"}
// Dependencies: {}
# [doc = " Return the current compressed (actual) read or write offset of `file`.  This"] # [doc = " offset includes the count of bytes that precede the gzip stream, for example"] # [doc = " when appending or when using [`gzdopen`] for reading. When reading, the"] # [doc = " offset does not include as yet unused buffered input. This information can"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * The number of bytes prior to the current read or write position in the"] # [doc = "   compressed data stream, on success."] # [doc = " * -1 on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzoffset64)] pub unsafe extern "C-unwind" fn gzoffset64 (file : gzFile) -> z_off64_t { let Some (state) = (unsafe { file . cast :: < GzState > () . as_ref () }) else { return - 1 ; } ; if state . mode != GzMode :: GZ_READ && state . mode != GzMode :: GZ_WRITE { return - 1 ; } let offset = lseek64 (state . fd , 0 , SEEK_CUR) as z_off64_t ; if offset == - 1 { return - 1 ; } match state . mode { GzMode :: GZ_READ => offset - state . stream . avail_in as z_off64_t , GzMode :: GZ_NONE | GzMode :: GZ_WRITE | GzMode :: GZ_APPEND => offset , } }
};
}
