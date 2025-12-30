// Generated macro for macro_504 (macro)
macro_rules! Depcrate_ffi_taskmacro_504 {
() => {
// Module: crate::ffi::task
// Provides: {"macro_504"}
// Dependencies: {}
ffi_fn ! { # [doc = " Creates a new task executor."] # [doc = ""] # [doc = " To avoid a memory leak, the executor must eventually be consumed by"] # [doc = " `hyper_executor_free`."] fn hyper_executor_new () -> * const hyper_executor { Arc :: into_raw (hyper_executor :: new ()) } ?= ptr :: null () }
};
}
