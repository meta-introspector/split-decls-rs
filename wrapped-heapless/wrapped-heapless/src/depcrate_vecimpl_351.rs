// Generated macro for impl_351 (impl)
macro_rules! Depcrate_vecimpl_351 {
() => {
// Module: crate::vec
// Provides: {"impl_351"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > Drop for VecInner < T , LenT , S > { fn drop (& mut self) { let mut_slice = self . as_mut_slice () ; unsafe { ptr :: drop_in_place (mut_slice) } } }
};
}
