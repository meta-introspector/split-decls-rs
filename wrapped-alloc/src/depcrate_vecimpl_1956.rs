// Generated macro for impl_1956 (impl)
macro_rules! Depcrate_vecimpl_1956 {
() => {
// Module: crate::vec
// Provides: {"impl_1956"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , A : Allocator > IntoIterator for & 'a mut Vec < T , A > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
