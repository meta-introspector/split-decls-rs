// Generated macro for impl_254 (impl)
macro_rules! Depcrate_vecimpl_254 {
() => {
// Module: crate::vec
// Provides: {"impl_254"}
// Dependencies: {}
impl < 'a , T , A : Allocator > IntoIterator for & 'a Vec < T , A > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
