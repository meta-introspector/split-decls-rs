// Generated macro for impl_387 (impl)
macro_rules! Depcrate_vecimpl_387 {
() => {
// Module: crate::vec
// Provides: {"impl_387"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > borrow :: BorrowMut < [T] > for VecInner < T , LenT , S > { fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
};
}
