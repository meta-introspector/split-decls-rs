// Generated macro for impl_1061 (impl)
macro_rules! Depcrate_iter_skip_any_whileimpl_1061 {
() => {
// Module: crate::iter::skip_any_while
// Provides: {"impl_1061"}
// Dependencies: {}
impl < I , P > ParallelIterator for SkipAnyWhile < I , P > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync + Send , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = SkipAnyWhileConsumer { base : consumer , predicate : & self . predicate , skipping : & AtomicBool :: new (true) , } ; self . base . drive_unindexed (consumer1) } }
};
}
