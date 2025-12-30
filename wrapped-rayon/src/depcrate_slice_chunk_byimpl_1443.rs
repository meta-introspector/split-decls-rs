// Generated macro for impl_1443 (impl)
macro_rules! Depcrate_slice_chunk_byimpl_1443 {
() => {
// Module: crate::slice::chunk_by
// Provides: {"impl_1443"}
// Dependencies: {}
impl < T : fmt :: Debug , P > fmt :: Debug for ChunkBy < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ChunkBy") . field ("slice" , & self . slice) . finish () } }
};
}
