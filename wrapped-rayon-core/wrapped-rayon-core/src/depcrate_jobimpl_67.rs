// Generated macro for impl_67 (impl)
macro_rules! Depcrate_jobimpl_67 {
() => {
// Module: crate::job
// Provides: {"impl_67"}
// Dependencies: {}
impl < L , F , R > Job for StackJob < L , F , R > where L : Latch + Sync , F : FnOnce (bool) -> R + Send , R : Send , { unsafe fn execute (this : * const ()) { unsafe { let this = & * (this as * const Self) ; let abort = unwind :: AbortIfPanic ; let func = (* this . func . get ()) . take () . unwrap () ; (* this . result . get ()) = JobResult :: call (func) ; Latch :: set (& this . latch) ; mem :: forget (abort) ; } } }
};
}
