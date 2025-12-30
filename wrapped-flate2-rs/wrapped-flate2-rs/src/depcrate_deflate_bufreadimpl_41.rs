// Generated macro for impl_41 (impl)
macro_rules! Depcrate_deflate_bufreadimpl_41 {
() => {
// Module: crate::deflate::bufread
// Provides: {"impl_41"}
// Dependencies: {}
impl < R : BufRead > DeflateEncoder < R > { # [doc = " Creates a new encoder which will read uncompressed data from the given"] # [doc = " stream and emit the compressed stream."] pub fn new (r : R , level : crate :: Compression) -> DeflateEncoder < R > { DeflateEncoder { obj : r , data : Compress :: new (level , false) , } } }
};
}
