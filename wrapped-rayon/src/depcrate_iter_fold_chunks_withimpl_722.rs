// Generated macro for impl_722 (impl)
macro_rules! Depcrate_iter_fold_chunks_withimpl_722 {
() => {
// Module: crate::iter::fold_chunks_with
// Provides: {"impl_722"}
// Dependencies: {}
impl < I : Debug , U : Debug , F > Debug for FoldChunksWith < I , U , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Fold") . field ("base" , & self . base) . field ("chunk_size" , & self . chunk_size) . field ("item" , & self . item) . finish () } }
};
}
