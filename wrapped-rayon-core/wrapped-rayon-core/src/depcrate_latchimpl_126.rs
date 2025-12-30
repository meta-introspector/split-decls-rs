// Generated macro for impl_126 (impl)
macro_rules! Depcrate_latchimpl_126 {
() => {
// Module: crate::latch
// Provides: {"impl_126"}
// Dependencies: {}
impl Latch for SpinLatch < '_ > { # [inline] unsafe fn set (this : * const Self) { unsafe { let registry : & Registry = if (* this) . cross { & Arc :: clone ((* this) . registry) } else { (* this) . registry } ; let target_worker_index = (* this) . target_worker_index ; if CoreLatch :: set (& (* this) . core_latch) { registry . notify_worker_latch_is_set (target_worker_index) ; } } } }
};
}
