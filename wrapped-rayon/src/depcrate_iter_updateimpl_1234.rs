// Generated macro for impl_1234 (impl)
macro_rules! Depcrate_iter_updateimpl_1234 {
() => {
// Module: crate::iter::update
// Provides: {"impl_1234"}
// Dependencies: {}
impl < I , F > ParallelIterator for Update < I , F > where I : ParallelIterator , F : Fn (& mut I :: Item) + Send + Sync , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer1 = UpdateConsumer :: new (consumer , & self . update_op) ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
};
}
