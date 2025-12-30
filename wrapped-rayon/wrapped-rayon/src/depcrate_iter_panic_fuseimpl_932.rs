// Generated macro for impl_932 (impl)
macro_rules! Depcrate_iter_panic_fuseimpl_932 {
() => {
// Module: crate::iter::panic_fuse
// Provides: {"impl_932"}
// Dependencies: {}
impl < I > ParallelIterator for PanicFuse < I > where I : ParallelIterator , { type Item = I :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let panicked = AtomicBool :: new (false) ; let consumer1 = PanicFuseConsumer { base : consumer , fuse : Fuse (& panicked) , } ; self . base . drive_unindexed (consumer1) } fn opt_len (& self) -> Option < usize > { self . base . opt_len () } }
};
}
