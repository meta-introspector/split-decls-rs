// Generated macro for impl_59 (impl)
macro_rules! Depcrate_astimpl_59 {
() => {
// Module: crate::ast
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a , A > IntoIterator for & 'a AttrList < A > { type Item = & 'a AList < A > ; type IntoIter = std :: slice :: Iter < 'a , AList < A > > ; fn into_iter (self) -> Self :: IntoIter { self . elems . iter () } }
};
}
