// Generated macro for impl_1447 (impl)
macro_rules! Depcrate_slice_chunk_byimpl_1447 {
() => {
// Module: crate::slice::chunk_by
// Provides: {"impl_1447"}
// Dependencies: {}
impl < T : fmt :: Debug , P > fmt :: Debug for ChunkByMut < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ChunkByMut") . field ("slice" , & self . slice) . finish () } }
};
}
