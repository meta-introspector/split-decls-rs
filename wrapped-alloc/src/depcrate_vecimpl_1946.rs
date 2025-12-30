// Generated macro for impl_1946 (impl)
macro_rules! Depcrate_vecimpl_1946 {
() => {
// Module: crate::vec
// Provides: {"impl_1946"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > ops :: Deref for Vec < T , A > { type Target = [T] ; # [inline] fn deref (& self) -> & [T] { self . as_slice () } }
};
}
