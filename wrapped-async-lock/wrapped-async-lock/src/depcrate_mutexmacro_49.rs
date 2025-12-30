// Generated macro for macro_49 (macro)
macro_rules! Depcrate_mutexmacro_49 {
() => {
// Module: crate::mutex
// Provides: {"macro_49"}
// Dependencies: {}
easy_wrapper ! { # [doc = " The future returned by [`Mutex::lock_arc`]."] pub struct LockArc < T : ? Sized > (LockArcInnards < T > => MutexGuardArc < T >) ; # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub (crate) wait () ; }
};
}
