// Generated macro for impl_1955 (impl)
macro_rules! Depcrate_vecimpl_1955 {
() => {
// Module: crate::vec
// Provides: {"impl_1955"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , A : Allocator > IntoIterator for & 'a Vec < T , A > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
