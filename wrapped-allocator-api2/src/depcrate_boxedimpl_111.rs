// Generated macro for impl_111 (impl)
macro_rules! Depcrate_boxedimpl_111 {
() => {
// Module: crate::boxed
// Provides: {"impl_111"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [cfg (feature = "std")] impl Clone for Box < std :: ffi :: CStr > { # [inline] fn clone (& self) -> Self { (* * self) . into () } }
};
}
