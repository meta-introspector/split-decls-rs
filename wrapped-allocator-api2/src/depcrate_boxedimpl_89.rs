// Generated macro for impl_89 (impl)
macro_rules! Depcrate_boxedimpl_89 {
() => {
// Module: crate::boxed
// Provides: {"impl_89"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > Deref for Box < T , A > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { unsafe { self . 0 . as_ref () } } }
};
}
