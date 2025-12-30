// Generated macro for impl_701 (impl)
macro_rules! Depcrate_iter_foldimpl_701 {
() => {
// Module: crate::iter::fold
// Provides: {"impl_701"}
// Dependencies: {}
impl < U , I , F > ParallelIterator for FoldWith < I , U , F > where I : ParallelIterator , F : Fn (U , I :: Item) -> U + Sync + Send , U : Send + Clone , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = FoldWithConsumer { base : consumer , item : self . item , fold_op : & self . fold_op , } ; self . base . drive_unindexed (consumer1) } }
};
}
