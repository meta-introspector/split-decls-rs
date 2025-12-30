// Generated macro for impl_13 (impl)
macro_rules! Depcrate_atomic_seq_lockimpl_13 {
() => {
// Module: crate::atomic::seq_lock
// Provides: {"impl_13"}
// Dependencies: {}
impl Drop for SeqLockWriteGuard { # [inline] fn drop (& mut self) { self . lock . state . store (self . state . wrapping_add (2) , Ordering :: Release) ; } }
};
}
