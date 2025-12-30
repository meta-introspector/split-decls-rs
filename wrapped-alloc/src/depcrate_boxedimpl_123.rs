// Generated macro for impl_123 (impl)
macro_rules! Depcrate_boxedimpl_123 {
() => {
// Module: crate::boxed
// Provides: {"impl_123"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl < T > Default for Box < [T] > { # [doc = " Creates an empty `[T]` inside a `Box`."] # [inline] fn default () -> Self { let ptr : Unique < [T] > = Unique :: < [T ; 0] > :: dangling () ; Box (ptr , Global) } }
};
}
