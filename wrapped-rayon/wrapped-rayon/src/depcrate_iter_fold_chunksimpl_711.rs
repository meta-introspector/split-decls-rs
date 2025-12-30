// Generated macro for impl_711 (impl)
macro_rules! Depcrate_iter_fold_chunksimpl_711 {
() => {
// Module: crate::iter::fold_chunks
// Provides: {"impl_711"}
// Dependencies: {}
impl < I : Debug , ID , F > Debug for FoldChunks < I , ID , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Fold") . field ("base" , & self . base) . field ("chunk_size" , & self . chunk_size) . finish () } }
};
}
