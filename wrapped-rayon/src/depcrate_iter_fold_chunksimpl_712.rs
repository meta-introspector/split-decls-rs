// Generated macro for impl_712 (impl)
macro_rules! Depcrate_iter_fold_chunksimpl_712 {
() => {
// Module: crate::iter::fold_chunks
// Provides: {"impl_712"}
// Dependencies: {}
impl < I , ID , F > FoldChunks < I , ID , F > { # [doc = " Creates a new `FoldChunks` iterator"] pub (super) fn new (base : I , chunk_size : usize , identity : ID , fold_op : F) -> Self { FoldChunks { base , chunk_size , identity , fold_op , } } }
};
}
