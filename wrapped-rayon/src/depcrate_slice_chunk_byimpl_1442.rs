// Generated macro for impl_1442 (impl)
macro_rules! Depcrate_slice_chunk_byimpl_1442 {
() => {
// Module: crate::slice::chunk_by
// Provides: {"impl_1442"}
// Dependencies: {}
impl < T , P : Clone > Clone for ChunkBy < '_ , T , P > { fn clone (& self) -> Self { ChunkBy { pred : self . pred . clone () , slice : self . slice , } } }
};
}
