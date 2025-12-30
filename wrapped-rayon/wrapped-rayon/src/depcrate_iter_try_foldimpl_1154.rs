// Generated macro for impl_1154 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1154 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1154"}
// Dependencies: {}
impl < I , U : Try , F > TryFoldWith < I , U , F > { pub (super) fn new (base : I , item : U :: Output , fold_op : F) -> Self { TryFoldWith { base , item , fold_op , } } }
};
}
