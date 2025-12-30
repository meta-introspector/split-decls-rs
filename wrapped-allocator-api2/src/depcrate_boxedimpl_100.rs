// Generated macro for impl_100 (impl)
macro_rules! Depcrate_boxedimpl_100 {
() => {
// Module: crate::boxed
// Provides: {"impl_100"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > borrow :: BorrowMut < T > for Box < T , A > { # [inline (always)] fn borrow_mut (& mut self) -> & mut T { self } }
};
}
