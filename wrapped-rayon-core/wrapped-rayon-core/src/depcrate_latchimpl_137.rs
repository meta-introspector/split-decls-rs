// Generated macro for impl_137 (impl)
macro_rules! Depcrate_latchimpl_137 {
() => {
// Module: crate::latch
// Provides: {"impl_137"}
// Dependencies: {}
impl Latch for CountLatch { # [inline] unsafe fn set (this : * const Self) { unsafe { if (* this) . counter . fetch_sub (1 , Ordering :: SeqCst) == 1 { match (* this) . kind { CountLatchKind :: Stealing { ref latch , ref registry , worker_index , } => { let registry = Arc :: clone (registry) ; if CoreLatch :: set (latch) { registry . notify_worker_latch_is_set (worker_index) ; } } CountLatchKind :: Blocking { ref latch } => LockLatch :: set (latch) , } } } } }
};
}
