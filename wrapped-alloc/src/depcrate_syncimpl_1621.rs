// Generated macro for impl_1621 (impl)
macro_rules! Depcrate_syncimpl_1621 {
() => {
// Module: crate::sync
// Provides: {"impl_1621"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "pin_default_impls" , since = "1.91.0")] impl < T > Default for Pin < Arc < T > > where T : ? Sized , Arc < T > : Default , { # [inline] fn default () -> Self { unsafe { Pin :: new_unchecked (Arc :: < T > :: default ()) } } }
};
}
