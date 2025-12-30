// Generated macro for impl_124 (impl)
macro_rules! Depcrate_boxedimpl_124 {
() => {
// Module: crate::boxed
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "default_box_extra" , since = "1.17.0")] impl Default for Box < str > { # [inline] fn default () -> Self { let ptr : Unique < str > = unsafe { let bytes : Unique < [u8] > = Unique :: < [u8 ; 0] > :: dangling () ; Unique :: new_unchecked (bytes . as_ptr () as * mut str) } ; Box (ptr , Global) } }
};
}
