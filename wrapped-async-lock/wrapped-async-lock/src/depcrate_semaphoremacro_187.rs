// Generated macro for macro_187 (macro)
macro_rules! Depcrate_semaphoremacro_187 {
() => {
// Module: crate::semaphore
// Provides: {"macro_187"}
// Dependencies: {}
easy_wrapper ! { # [doc = " The future returned by [`Semaphore::acquire_arc`]."] pub struct AcquireArc (AcquireArcInner => SemaphoreGuardArc) ; # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub (crate) wait () ; }
};
}
