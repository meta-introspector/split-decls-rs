// Generated macro for impl_385 (impl)
macro_rules! Depcrate_vecimpl_385 {
() => {
// Module: crate::vec
// Provides: {"impl_385"}
// Dependencies: {}
impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > ops :: DerefMut for VecInner < T , LenT , S > { fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
};
}
