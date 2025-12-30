// Generated macro for impl_48 (impl)
macro_rules! Depcrate_deflate_bufreadimpl_48 {
() => {
// Module: crate::deflate::bufread
// Provides: {"impl_48"}
// Dependencies: {}
impl < R : BufRead > DeflateDecoder < R > { # [doc = " Creates a new decoder which will decompress data read from the given"] # [doc = " stream."] pub fn new (r : R) -> DeflateDecoder < R > { DeflateDecoder { obj : r , data : Decompress :: new (false) , } } }
};
}
