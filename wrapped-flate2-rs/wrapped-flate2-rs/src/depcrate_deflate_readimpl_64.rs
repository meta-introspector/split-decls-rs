// Generated macro for impl_64 (impl)
macro_rules! Depcrate_deflate_readimpl_64 {
() => {
// Module: crate::deflate::read
// Provides: {"impl_64"}
// Dependencies: {}
impl < R : Read > DeflateDecoder < R > { # [doc = " Creates a new decoder which will decompress data read from the given"] # [doc = " stream."] pub fn new (r : R) -> DeflateDecoder < R > { DeflateDecoder :: new_with_buf (r , vec ! [0 ; 32 * 1024]) } # [doc = " Same as `new`, but the intermediate buffer for data is specified."] # [doc = ""] # [doc = " Note that the capacity of the intermediate buffer is never increased,"] # [doc = " and it is recommended for it to be large."] pub fn new_with_buf (r : R , buf : Vec < u8 >) -> DeflateDecoder < R > { DeflateDecoder { inner : bufread :: DeflateDecoder :: new (BufReader :: with_buf (buf , r)) , } } }
};
}
