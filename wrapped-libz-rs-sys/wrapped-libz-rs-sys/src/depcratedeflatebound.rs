// Generated macro for deflateBound (function)
macro_rules! DepcratedeflateBound {
() => {
// Module: crate
// Provides: {"deflateBound"}
// Dependencies: {}
# [doc = " Returns an upper bound on the compressed size after deflation of `sourceLen` bytes."] # [doc = ""] # [doc = " This function must be called after [`deflateInit_`] or [`deflateInit2_`]."] # [doc = " This would be used to allocate an output buffer for deflation in a single pass, and so would be called before [`deflate`]."] # [doc = " If that first [`deflate`] call is provided the `sourceLen` input bytes, an output buffer allocated to the size returned by [`deflateBound`],"] # [doc = " and the flush value [`Z_FINISH`], then [`deflate`] is guaranteed to return [`Z_STREAM_END`]."] # [doc = ""] # [doc = " Note that it is possible for the compressed size to be larger than the value returned by [`deflateBound`]"] # [doc = " if flush options other than [`Z_FINISH`] or [`Z_NO_FLUSH`] are used."] # [doc = ""] # [doc = " ## Safety"] # [doc = ""] # [doc = " * Either"] # [doc = "     - `strm` is `NULL`"] # [doc = "     - `strm` satisfies the requirements of `&mut *strm` and was initialized with [`deflateInit_`] or similar"] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (deflateBound))] pub unsafe extern "C-unwind" fn deflateBound (strm : * mut z_stream , sourceLen : c_ulong) -> c_ulong { zlib_rs :: deflate :: bound (DeflateStream :: from_stream_mut (strm) , sourceLen as usize) as c_ulong }
};
}
