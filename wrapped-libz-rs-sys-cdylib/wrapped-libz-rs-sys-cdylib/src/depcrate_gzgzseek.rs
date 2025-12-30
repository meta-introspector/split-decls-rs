// Generated macro for gzseek (function)
macro_rules! Depcrate_gzgzseek {
() => {
// Module: crate::gz
// Provides: {"gzseek"}
// Dependencies: {}
# [doc = " Set the starting position to `offset` relative to `whence` for the next [`gzread`]"] # [doc = " or [`gzwrite`] on `file`. The `offset` represents a number of bytes in the"] # [doc = " uncompressed data stream. The `whence` parameter is defined as in `lseek(2)`,"] # [doc = " but only `SEEK_CUR` (relative to current position) and `SEEK_SET` (absolute from"] # [doc = " start of the uncompressed data stream) are supported."] # [doc = ""] # [doc = " If `file` is open for reading, this function is emulated but can extremely"] # [doc = " slow (because it operates on the decompressed data stream).  If `file` is open"] # [doc = " for writing, only forward seeks are supported; `gzseek` then compresses a sequence"] # [doc = " of zeroes up to the new starting position. If a negative `offset` is specified in"] # [doc = " write mode, `gzseek` returns -1."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - The resulting offset location as measured in bytes from the beginning of the uncompressed"] # [doc = "   stream, on success."] # [doc = " - `-1` on error."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `file`, if non-null, must be an open file handle obtained from [`gzopen`] or [`gzdopen`]."] # [export_name = crate :: prefix ! (gzseek)] pub unsafe extern "C-unwind" fn gzseek (file : gzFile , offset : z_off_t , whence : c_int) -> z_off_t { z_off_t :: try_from (unsafe { gzseek64 (file , offset as z_off64_t , whence) }) . unwrap_or (- 1) }
};
}
