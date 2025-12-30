// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_iter_take_any_whileimpl_1131 {
() => {
// Module: crate::iter::take_any_while
// Provides: {"impl_1131"}
// Dependencies: {}
impl < I , P > ParallelIterator for TakeAnyWhile < I , P > where I : ParallelIterator , P : Fn (& I :: Item) -> bool + Sync + Send , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = TakeAnyWhileConsumer { base : consumer , predicate : & self . predicate , taking : & AtomicBool :: new (true) , } ; self . base . drive_unindexed (consumer1) } }
};
}
