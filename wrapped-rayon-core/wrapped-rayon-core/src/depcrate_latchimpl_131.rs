// Generated macro for impl_131 (impl)
macro_rules! Depcrate_latchimpl_131 {
() => {
// Module: crate::latch
// Provides: {"impl_131"}
// Dependencies: {}
impl OnceLatch { # [inline] pub (super) fn new () -> OnceLatch { Self { core_latch : CoreLatch :: new () , } } # [doc = " Set the latch, then tickle the specific worker thread,"] # [doc = " which should be the one that owns this latch."] # [inline] pub (super) unsafe fn set_and_tickle_one (this : * const Self , registry : & Registry , target_worker_index : usize ,) { unsafe { if CoreLatch :: set (& (* this) . core_latch) { registry . notify_worker_latch_is_set (target_worker_index) ; } } } }
};
}
