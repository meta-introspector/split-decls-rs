// Generated macro for impl_405 (impl)
macro_rules! Depcrate_iter_clonedimpl_405 {
() => {
// Module: crate::iter::cloned
// Provides: {"impl_405"}
// Dependencies: {}
impl < 'a , T , I > ParallelIterator for Cloned < I > where I : ParallelIterator < Item = & 'a T > , T : 'a + Clone + Send + Sync , { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = ClonedConsumer :: new (consumer) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
};
}
