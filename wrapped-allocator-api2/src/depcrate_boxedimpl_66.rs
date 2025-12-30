// Generated macro for impl_66 (impl)
macro_rules! Depcrate_boxedimpl_66 {
() => {
// Module: crate::boxed
// Provides: {"impl_66"}
// Dependencies: {}
impl < A : Allocator + Default > Default for Box < str , A > { # [inline (always)] fn default () -> Self { let ptr : Unique < str > = unsafe { let bytes : NonNull < [u8] > = NonNull :: < [u8 ; 0] > :: dangling () ; Unique :: new_unchecked (bytes . as_ptr () as * mut str) } ; Box (ptr , A :: default ()) } }
};
}
