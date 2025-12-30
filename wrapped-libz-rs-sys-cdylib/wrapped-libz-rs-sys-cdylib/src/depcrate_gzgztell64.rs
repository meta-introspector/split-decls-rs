// Generated macro for gztell64 (function)
macro_rules! Depcrate_gzgztell64 {
() => {
// Module: crate::gz
// Provides: {"gztell64"}
// Dependencies: {}
# [doc = " Return the starting position for the next [`gzread`] or [`gzwrite`] on `file`."] # [doc = " This position represents a number of bytes in the uncompressed data stream,"] # [doc = " and is zero when starting, even if appending or reading a gzip stream from"] # [doc = " the middle of a file using [`gzdopen`]."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * The number of bytes prior to the current read or write position in the"] # [doc = "   uncompressed data stream, on success."] # [doc = " * -1 on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gztell64)] pub unsafe extern "C-unwind" fn gztell64 (file : gzFile) -> z_off64_t { let Some (state) = (unsafe { file . cast :: < GzState > () . as_ref () }) else { return - 1 ; } ; if state . mode != GzMode :: GZ_READ && state . mode != GzMode :: GZ_WRITE { return - 1 ; } match state . seek { true => (state . pos + state . skip) as z_off64_t , false => state . pos as z_off64_t , } }
};
}
