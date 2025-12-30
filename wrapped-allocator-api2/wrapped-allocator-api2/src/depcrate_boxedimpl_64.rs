// Generated macro for impl_64 (impl)
macro_rules! Depcrate_boxedimpl_64 {
() => {
// Module: crate::boxed
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : Default > Default for Box < T > { # [doc = " Creates a `Box<T>`, with the `Default` value for T."] # [inline (always)] fn default () -> Self { Box :: new (T :: default ()) } }
};
}
