// Generated macro for impl_1592 (impl)
macro_rules! Depcrate_sliceimpl_1592 {
() => {
// Module: crate::slice
// Provides: {"impl_1592"}
// Dependencies: {}
impl < 'data , T : Send > ParallelIterator for IterMut < 'data , T > { type Item = & 'data mut T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
