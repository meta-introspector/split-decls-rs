// Generated macro for impl_502 (impl)
macro_rules! Depcrate_vector_rayonimpl_502 {
() => {
// Module: crate::vector::rayon
// Provides: {"impl_502"}
// Dependencies: {}
impl < 'a , A > ParallelIterator for ParIterMut < 'a , A > where A : Clone + Send + Sync + 'a , { type Item = & 'a mut A ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } }
};
}
