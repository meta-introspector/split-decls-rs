// Generated macro for macro_183 (macro)
macro_rules! Depcrate_semaphoremacro_183 {
() => {
// Module: crate::semaphore
// Provides: {"macro_183"}
// Dependencies: {}
easy_wrapper ! { # [doc = " The future returned by [`Semaphore::acquire`]."] pub struct Acquire <'a > (AcquireInner <'a > => SemaphoreGuard <'a >) ; # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub (crate) wait () ; }
};
}
