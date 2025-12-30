// Generated macro for impl_397 (impl)
macro_rules! Depcrate_iter_chunksimpl_397 {
() => {
// Module: crate::iter::chunks
// Provides: {"impl_397"}
// Dependencies: {}
impl < P > ExactSizeIterator for ChunkSeq < P > where P : Producer , { # [inline] fn len (& self) -> usize { self . len . div_ceil (self . chunk_size) } }
};
}
