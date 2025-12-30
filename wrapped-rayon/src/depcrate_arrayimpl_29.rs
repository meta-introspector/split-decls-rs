// Generated macro for impl_29 (impl)
macro_rules! Depcrate_arrayimpl_29 {
() => {
// Module: crate::array
// Provides: {"impl_29"}
// Dependencies: {}
impl < T : Send , const N : usize > ParallelIterator for IntoIter < T , N > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (N) } }
};
}
