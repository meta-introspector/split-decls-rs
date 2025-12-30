// Generated macro for gzprintf (function)
macro_rules! Depcrate_gzgzprintf {
() => {
// Module: crate::gz
// Provides: {"gzprintf"}
// Dependencies: {}
# [doc = " Convert, format, compress, and write the variadic arguments `...` to a file under control of the string format, as in `fprintf`."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " Returns the number of uncompressed bytes actually written, or a negative zlib error code in case of error."] # [doc = " The number of uncompressed bytes written is limited to 8191, or one less than the buffer size given to [`gzbuffer`]."] # [doc = " The caller should assure that this limit is not exceeded. If it is exceeded, then [`gzprintf`] will return `0` with nothing written."] # [doc = ""] # [doc = " Contrary to other implementations that can use the insecure `vsprintf`, the `zlib-rs` library always uses `vsnprintf`,"] # [doc = " so attempting to write more bytes than the limit can never run into buffer overflow issues."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - The `format`  must be a valid C string"] # [doc = " - The variadic arguments must correspond with the format string in number and type"] # [cfg (feature = "gzprintf")] # [export_name = crate :: prefix ! (gzprintf)] pub unsafe extern "C-unwind" fn gzprintf (file : gzFile , format : * const c_char , mut va : ...) -> c_int { unsafe { gzvprintf (file , format , va . as_va_list ()) } }
};
}
