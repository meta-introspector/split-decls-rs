// Generated macro for impl_99 (impl)
macro_rules! Depcrate_boxedimpl_99 {
() => {
// Module: crate::boxed
// Provides: {"impl_99"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > borrow :: Borrow < T > for Box < T , A > { # [inline (always)] fn borrow (& self) -> & T { self } }
};
}
