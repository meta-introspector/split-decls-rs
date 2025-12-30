// Generated macro for impl_1157 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1157 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1157"}
// Dependencies: {}
impl < U , I , F > ParallelIterator for TryFoldWith < I , U , F > where I : ParallelIterator , F : Fn (U :: Output , I :: Item) -> U + Sync + Send , U : Try < Output : Clone + Send > + Send , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = TryFoldWithConsumer { base : consumer , item : self . item , fold_op : & self . fold_op , } ; self . base . drive_unindexed (consumer1) } }
};
}
