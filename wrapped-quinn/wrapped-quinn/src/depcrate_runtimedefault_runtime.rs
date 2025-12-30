// Generated macro for default_runtime (function)
macro_rules! Depcrate_runtimedefault_runtime {
() => {
// Module: crate::runtime
// Provides: {"default_runtime"}
// Dependencies: {}
# [doc = " Automatically select an appropriate runtime from those enabled at compile time"] # [doc = ""] # [doc = " If `runtime-tokio` is enabled and this function is called from within a Tokio runtime context,"] # [doc = " then `TokioRuntime` is returned. Otherwise, if `runtime-smol` is enabled, `SmolRuntime` is"] # [doc = " returned. Otherwise, `None` is returned."] # [allow (clippy :: needless_return)] pub fn default_runtime () -> Option < Arc < dyn Runtime > > { # [cfg (feature = "runtime-tokio")] { if :: tokio :: runtime :: Handle :: try_current () . is_ok () { return Some (Arc :: new (TokioRuntime)) ; } } # [cfg (feature = "runtime-smol")] { return Some (Arc :: new (SmolRuntime)) ; } # [cfg (not (feature = "runtime-smol"))] None }
};
}
