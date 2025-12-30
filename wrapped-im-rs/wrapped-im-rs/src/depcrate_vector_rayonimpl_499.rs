// Generated macro for impl_499 (impl)
macro_rules! Depcrate_vector_rayonimpl_499 {
() => {
// Module: crate::vector::rayon
// Provides: {"impl_499"}
// Dependencies: {}
impl < 'a , A > ParallelIterator for ParIter < 'a , A > where A : Clone + Send + Sync + 'a , { type Item = & 'a A ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } }
};
}
