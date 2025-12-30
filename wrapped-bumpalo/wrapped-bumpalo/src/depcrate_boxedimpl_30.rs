// Generated macro for impl_30 (impl)
macro_rules! Depcrate_boxedimpl_30 {
() => {
// Module: crate::boxed
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , T : ? Sized > borrow :: BorrowMut < T > for Box < 'a , T > { fn borrow_mut (& mut self) -> & mut T { & mut * * self } }
};
}
