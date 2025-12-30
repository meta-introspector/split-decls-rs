// Generated macro for impl_278 (impl)
macro_rules! Depcrate_ord_setimpl_278 {
() => {
// Module: crate::ord::set
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'a , A > IntoIterator for & 'a OrdSet < A > where A : 'a + Ord , { type Item = & 'a A ; type IntoIter = Iter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
