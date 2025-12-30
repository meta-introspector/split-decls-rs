// Generated macro for impl_59 (impl)
macro_rules! Depcrate_deflate_readimpl_59 {
() => {
// Module: crate::deflate::read
// Provides: {"impl_59"}
// Dependencies: {}
impl < R : Read > DeflateEncoder < R > { # [doc = " Creates a new encoder which will read uncompressed data from the given"] # [doc = " stream and emit the compressed stream."] pub fn new (r : R , level : crate :: Compression) -> DeflateEncoder < R > { DeflateEncoder { inner : bufread :: DeflateEncoder :: new (BufReader :: new (r) , level) , } } }
};
}
