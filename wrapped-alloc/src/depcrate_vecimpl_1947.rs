// Generated macro for impl_1947 (impl)
macro_rules! Depcrate_vecimpl_1947 {
() => {
// Module: crate::vec
// Provides: {"impl_1947"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > ops :: DerefMut for Vec < T , A > { # [inline] fn deref_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
};
}
