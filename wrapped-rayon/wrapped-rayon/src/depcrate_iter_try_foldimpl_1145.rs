// Generated macro for impl_1145 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1145 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1145"}
// Dependencies: {}
impl < I , U , ID , F > TryFold < I , U , ID , F > { pub (super) fn new (base : I , identity : ID , fold_op : F) -> Self { TryFold { base , identity , fold_op , marker : PhantomData , } } }
};
}
