// Generated macro for impl_62 (impl)
macro_rules! Depcrate_atomicimpl_62 {
() => {
// Module: crate::atomic
// Provides: {"impl_62"}
// Dependencies: {}
impl < T : ? Sized + Pointable > BorrowMut < T > for Owned < T > { fn borrow_mut (& mut self) -> & mut T { self . deref_mut () } }
};
}
