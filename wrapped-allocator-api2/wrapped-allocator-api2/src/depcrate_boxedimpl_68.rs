// Generated macro for impl_68 (impl)
macro_rules! Depcrate_boxedimpl_68 {
() => {
// Module: crate::boxed
// Provides: {"impl_68"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl Clone for Box < str > { # [inline (always)] fn clone (& self) -> Self { let buf : Box < [u8] > = self . as_bytes () . into () ; unsafe { Box :: from_raw (Box :: into_raw (buf) as * mut str) } } }
};
}
