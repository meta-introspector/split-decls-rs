// Generated macro for impl_65 (impl)
macro_rules! Depcrate_boxedimpl_65 {
() => {
// Module: crate::boxed
// Provides: {"impl_65"}
// Dependencies: {}
impl < T , A : Allocator + Default > Default for Box < [T] , A > { # [inline (always)] fn default () -> Self { let ptr : NonNull < [T] > = NonNull :: < [T ; 0] > :: dangling () ; Box (unsafe { Unique :: new_unchecked (ptr . as_ptr ()) } , A :: default ()) } }
};
}
