// Generated macro for impl_1148 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1148 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1148"}
// Dependencies: {}
impl < U , I , ID , F > ParallelIterator for TryFold < I , U , ID , F > where I : ParallelIterator , F : Fn (U :: Output , I :: Item) -> U + Sync + Send , ID : Fn () -> U :: Output + Sync + Send , U : Try + Send , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = TryFoldConsumer { base : consumer , identity : & self . identity , fold_op : & self . fold_op , marker : PhantomData , } ; self . base . drive_unindexed (consumer1) } }
};
}
