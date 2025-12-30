// Generated macro for impl_1248 (impl)
macro_rules! Depcrate_graphmapimpl_1248 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1248"}
// Dependencies: {}
# [cfg (feature = "rayon")] impl < N > ParallelIterator for ParNodes < '_ , N > where N : NodeTrait + Send + Sync , { type Item = N ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : rayon :: iter :: plumbing :: UnindexedConsumer < Self :: Item > , { self . iter . copied () . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { self . iter . opt_len () } }
};
}
