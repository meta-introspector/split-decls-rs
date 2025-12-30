// Generated macro for impl_779 (impl)
macro_rules! Depcrate_iter_inspectimpl_779 {
() => {
// Module: crate::iter::inspect
// Provides: {"impl_779"}
// Dependencies: {}
impl < I , F > ParallelIterator for Inspect < I , F > where I : ParallelIterator , F : Fn (& I :: Item) + Sync + Send , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = InspectConsumer :: new (consumer , & self . inspect_op) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
};
}
