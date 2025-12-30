// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_iter_step_byimpl_1084 {
() => {
// Module: crate::iter::step_by
// Provides: {"impl_1084"}
// Dependencies: {}
impl < I > ParallelIterator for StepBy < I > where I : IndexedParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
