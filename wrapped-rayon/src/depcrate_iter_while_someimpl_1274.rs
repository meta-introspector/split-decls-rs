// Generated macro for impl_1274 (impl)
macro_rules! Depcrate_iter_while_someimpl_1274 {
() => {
// Module: crate::iter::while_some
// Provides: {"impl_1274"}
// Dependencies: {}
impl < I , T > ParallelIterator for WhileSome < I > where I : ParallelIterator < Item = Option < T > > , T : Send , { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let full = AtomicBool :: new (false) ; let consumer1 = WhileSomeConsumer { base : consumer , full : & full , } ; self . base . drive_unindexed (consumer1) } }
};
}
