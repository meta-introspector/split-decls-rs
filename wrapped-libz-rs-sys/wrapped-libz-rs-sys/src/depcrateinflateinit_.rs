// Generated macro for inflateInit_ (function)
macro_rules! DepcrateinflateInit_ {
() => {
// Module: crate
// Provides: {"inflateInit_"}
// Dependencies: {}
# [doc = " Initializes the state for decompression"] # [doc = ""] # [doc = " A call to [`inflateInit_`] is equivalent to [`inflateInit2_`] where `windowBits` is 15."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - [`Z_OK`] if success"] # [doc = " - [`Z_MEM_ERROR`] if there was not enough memory"] # [doc = " - [`Z_VERSION_ERROR`] if the zlib library version is incompatible with the version assumed by the caller"] # [doc = " - [`Z_STREAM_ERROR`] if a parameter is invalid, such as a null pointer to the structure"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee that"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm`"] # [doc = " * Either"] # [doc = "     - `version` is NULL"] # [doc = "     - `version` satisfies the requirements of [`core::ffi::CStr::from_ptr`]"] # [doc = " * If `strm` is not `NULL`, the following fields contain valid values"] # [doc = "     - `zalloc`"] # [doc = "     - `zfree`"] # [doc = "     - `opaque`"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (inflateInit_))] pub unsafe extern "C-unwind" fn inflateInit_ (strm : z_streamp , version : * const c_char , stream_size : c_int ,) -> c_int { let config = InflateConfig :: default () ; unsafe { inflateInit2_ (strm , config . window_bits , version , stream_size) } }
};
}
