// Generated macro for impl_247 (impl)
macro_rules! Depcrate_vecimpl_247 {
() => {
// Module: crate::vec
// Provides: {"impl_247"}
// Dependencies: {}
impl < T , A : Allocator > ops :: DerefMut for Vec < T , A > { # [inline (always)] fn deref_mut (& mut self) -> & mut [T] { unsafe { slice :: from_raw_parts_mut (self . as_mut_ptr () , self . len) } } }
};
}
