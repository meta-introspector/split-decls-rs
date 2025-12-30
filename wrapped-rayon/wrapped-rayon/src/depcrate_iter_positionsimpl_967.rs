// Generated macro for impl_967 (impl)
macro_rules! Depcrate_iter_positionsimpl_967 {
() => {
// Module: crate::iter::positions
// Provides: {"impl_967"}
// Dependencies: {}
impl < I , P > ParallelIterator for Positions < I , P > where I : IndexedParallelIterator , P : Fn (I :: Item) -> bool + Sync + Send , { type Item = usize ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = PositionsConsumer :: new (consumer , & self . predicate , 0) ; self . base . drive (consumer1) } }
};
}
