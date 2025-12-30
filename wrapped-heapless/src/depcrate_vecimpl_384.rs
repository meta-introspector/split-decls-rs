// Generated macro for impl_384 (impl)
macro_rules! Depcrate_vecimpl_384 {
() => {
// Module: crate::vec
// Provides: {"impl_384"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > ops :: Deref for VecInner < T , LenT , S > { type Target = [T] ; fn deref (& self) -> & Self :: Target { self . as_slice () } }
};
}
