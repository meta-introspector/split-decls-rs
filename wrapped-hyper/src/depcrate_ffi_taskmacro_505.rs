// Generated macro for macro_505 (macro)
macro_rules! Depcrate_ffi_taskmacro_505 {
() => {
// Module: crate::ffi::task
// Provides: {"macro_505"}
// Dependencies: {}
ffi_fn ! { # [doc = " Frees an executor and any incomplete tasks still part of it."] # [doc = ""] # [doc = " This should be used for any executor once it is no longer needed."] fn hyper_executor_free (exec : * const hyper_executor) { drop (non_null ! (Arc :: from_raw (exec) ?= ())) ; } }
};
}
