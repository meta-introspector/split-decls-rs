// Generated macro for impl_330 (impl)
macro_rules! Depcrate_zlib_readimpl_330 {
() => {
// Module: crate::zlib::read
// Provides: {"impl_330"}
// Dependencies: {}
impl < R : Read > ZlibEncoder < R > { # [doc = " Creates a new encoder which will read uncompressed data from the given"] # [doc = " stream and emit the compressed stream."] pub fn new (r : R , level : crate :: Compression) -> ZlibEncoder < R > { ZlibEncoder { inner : bufread :: ZlibEncoder :: new (BufReader :: new (r) , level) , } } # [doc = " Creates a new encoder with the given `compression` settings which will"] # [doc = " read uncompressed data from the given stream `r` and emit the compressed stream."] pub fn new_with_compress (r : R , compression : crate :: Compress) -> ZlibEncoder < R > { ZlibEncoder { inner : bufread :: ZlibEncoder :: new_with_compress (BufReader :: new (r) , compression) , } } }
};
}
