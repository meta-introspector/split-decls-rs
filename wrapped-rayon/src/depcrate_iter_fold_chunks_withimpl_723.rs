// Generated macro for impl_723 (impl)
macro_rules! Depcrate_iter_fold_chunks_withimpl_723 {
() => {
// Module: crate::iter::fold_chunks_with
// Provides: {"impl_723"}
// Dependencies: {}
impl < I , U , F > FoldChunksWith < I , U , F > { # [doc = " Creates a new `FoldChunksWith` iterator"] pub (super) fn new (base : I , chunk_size : usize , item : U , fold_op : F) -> Self { FoldChunksWith { base , chunk_size , item , fold_op , } } }
};
}
