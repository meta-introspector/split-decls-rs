// Generated macro for impl_128 (impl)
macro_rules! Depcrate_boxedimpl_128 {
() => {
// Module: crate::boxed
// Provides: {"impl_128"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "box_slice_clone" , since = "1.3.0")] impl Clone for Box < str > { fn clone (& self) -> Self { let buf : Box < [u8] > = self . as_bytes () . into () ; unsafe { from_boxed_utf8_unchecked (buf) } } }
};
}
