// Generated macro for impl_50 (impl)
macro_rules! Depcrate_implsimpl_50 {
() => {
// Module: crate::impls
// Provides: {"impl_50"}
// Dependencies: {}
impl < T , N : ArrayLength > BorrowMut < [T] > for GenericArray < T , N > { # [inline (always)] fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
};
}
