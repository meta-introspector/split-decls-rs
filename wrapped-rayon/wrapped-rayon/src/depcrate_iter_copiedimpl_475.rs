// Generated macro for impl_475 (impl)
macro_rules! Depcrate_iter_copiedimpl_475 {
() => {
// Module: crate::iter::copied
// Provides: {"impl_475"}
// Dependencies: {}
impl < 'a , T , I > ParallelIterator for Copied < I > where I : ParallelIterator < Item = & 'a T > , T : 'a + Copy + Send + Sync , { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = CopiedConsumer :: new (consumer) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
};
}
