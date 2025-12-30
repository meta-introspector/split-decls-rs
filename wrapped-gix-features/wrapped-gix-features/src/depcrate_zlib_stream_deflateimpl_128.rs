// Generated macro for impl_128 (impl)
macro_rules! Depcrate_zlib_stream_deflateimpl_128 {
() => {
// Module: crate::zlib::stream::deflate
// Provides: {"impl_128"}
// Dependencies: {}
impl < W > Clone for Write < W > where W : Clone , { fn clone (& self) -> Self { Write { compressor : impls :: new_compress () , inner : self . inner . clone () , buf : self . buf , } } }
};
}
