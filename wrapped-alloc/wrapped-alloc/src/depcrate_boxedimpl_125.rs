// Generated macro for impl_125 (impl)
macro_rules! Depcrate_boxedimpl_125 {
() => {
// Module: crate::boxed
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "pin_default_impls" , since = "1.91.0")] impl < T > Default for Pin < Box < T > > where T : ? Sized , Box < T > : Default , { # [inline] fn default () -> Self { Box :: into_pin (Box :: < T > :: default ()) } }
};
}
