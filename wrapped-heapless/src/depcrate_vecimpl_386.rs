// Generated macro for impl_386 (impl)
macro_rules! Depcrate_vecimpl_386 {
() => {
// Module: crate::vec
// Provides: {"impl_386"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > borrow :: Borrow < [T] > for VecInner < T , LenT , S > { fn borrow (& self) -> & [T] { self . as_slice () } }
};
}
