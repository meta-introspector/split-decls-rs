// Generated macro for impl_692 (impl)
macro_rules! Depcrate_iter_foldimpl_692 {
() => {
// Module: crate::iter::fold
// Provides: {"impl_692"}
// Dependencies: {}
impl < U , I , ID , F > ParallelIterator for Fold < I , ID , F > where I : ParallelIterator , F : Fn (U , I :: Item) -> U + Sync + Send , ID : Fn () -> U + Sync + Send , U : Send , { type Item = U ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = FoldConsumer { base : consumer , fold_op : & self . fold_op , identity : & self . identity , } ; self . base . drive_unindexed (consumer1) } }
};
}
