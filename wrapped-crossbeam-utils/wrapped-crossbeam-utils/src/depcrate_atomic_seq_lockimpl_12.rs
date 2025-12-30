// Generated macro for impl_12 (impl)
macro_rules! Depcrate_atomic_seq_lockimpl_12 {
() => {
// Module: crate::atomic::seq_lock
// Provides: {"impl_12"}
// Dependencies: {}
impl SeqLockWriteGuard { # [doc = " Releases the lock without incrementing the stamp."] # [inline] pub (crate) fn abort (self) { self . lock . state . store (self . state , Ordering :: Release) ; mem :: forget (self) ; } }
};
}
