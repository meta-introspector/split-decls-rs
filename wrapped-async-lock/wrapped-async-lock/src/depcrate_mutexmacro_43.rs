// Generated macro for macro_43 (macro)
macro_rules! Depcrate_mutexmacro_43 {
() => {
// Module: crate::mutex
// Provides: {"macro_43"}
// Dependencies: {}
easy_wrapper ! { # [doc = " The future returned by [`Mutex::lock`]."] pub struct Lock <'a , T : ? Sized > (LockInner <'a , T > => MutexGuard <'a , T >) ; # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub (crate) wait () ; }
};
}
