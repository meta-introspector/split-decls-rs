// Generated macro for impl_311 (impl)
macro_rules! Depcrate_zlib_bufreadimpl_311 {
() => {
// Module: crate::zlib::bufread
// Provides: {"impl_311"}
// Dependencies: {}
impl < R : BufRead > ZlibEncoder < R > { # [doc = " Creates a new encoder which will read uncompressed data from the given"] # [doc = " stream and emit the compressed stream."] pub fn new (r : R , level : crate :: Compression) -> ZlibEncoder < R > { ZlibEncoder { obj : r , data : Compress :: new (level , true) , } } # [doc = " Creates a new encoder with the given `compression` settings which will"] # [doc = " read uncompressed data from the given stream `r` and emit the compressed stream."] pub fn new_with_compress (r : R , compression : Compress) -> ZlibEncoder < R > { ZlibEncoder { obj : r , data : compression , } } }
};
}
