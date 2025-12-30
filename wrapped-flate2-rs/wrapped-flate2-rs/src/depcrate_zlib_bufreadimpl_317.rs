// Generated macro for impl_317 (impl)
macro_rules! Depcrate_zlib_bufreadimpl_317 {
() => {
// Module: crate::zlib::bufread
// Provides: {"impl_317"}
// Dependencies: {}
impl < R : BufRead > ZlibDecoder < R > { # [doc = " Creates a new decoder which will decompress data read from the given"] # [doc = " stream."] pub fn new (r : R) -> ZlibDecoder < R > { ZlibDecoder { obj : r , data : Decompress :: new (true) , } } # [doc = " Creates a new decoder which will decompress data read from the given"] # [doc = " stream, using the given `decompression` settings."] pub fn new_with_decompress (r : R , decompression : Decompress) -> ZlibDecoder < R > { ZlibDecoder { obj : r , data : decompression , } } }
};
}
