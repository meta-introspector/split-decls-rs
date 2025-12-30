// Generated macro for impl_90 (impl)
macro_rules! Depcrate_boxedimpl_90 {
() => {
// Module: crate::boxed
// Provides: {"impl_90"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > DerefMut for Box < T , A > { # [inline (always)] fn deref_mut (& mut self) -> & mut T { unsafe { self . 0 . as_mut () } } }
};
}
