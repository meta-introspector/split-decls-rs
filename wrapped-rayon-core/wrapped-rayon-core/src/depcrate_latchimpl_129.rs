// Generated macro for impl_129 (impl)
macro_rules! Depcrate_latchimpl_129 {
() => {
// Module: crate::latch
// Provides: {"impl_129"}
// Dependencies: {}
impl Latch for LockLatch { # [inline] unsafe fn set (this : * const Self) { unsafe { let mut guard = (* this) . m . lock () . unwrap () ; * guard = true ; (* this) . v . notify_all () ; } } }
};
}
