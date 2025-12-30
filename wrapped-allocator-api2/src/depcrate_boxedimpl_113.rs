// Generated macro for impl_113 (impl)
macro_rules! Depcrate_boxedimpl_113 {
() => {
// Module: crate::boxed
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [cfg (all (feature = "fresh-rust" , not (feature = "std")))] impl Clone for Box < core :: ffi :: CStr > { # [inline] fn clone (& self) -> Self { (* * self) . into () } }
};
}
