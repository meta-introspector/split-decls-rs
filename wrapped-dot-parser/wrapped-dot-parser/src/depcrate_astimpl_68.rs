// Generated macro for impl_68 (impl)
macro_rules! Depcrate_astimpl_68 {
() => {
// Module: crate::ast
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a , A > IntoIterator for & 'a AList < A > { type Item = & 'a A ; type IntoIter = std :: slice :: Iter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . elems . iter () } }
};
}
