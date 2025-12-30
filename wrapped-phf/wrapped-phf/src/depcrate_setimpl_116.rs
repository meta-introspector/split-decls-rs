// Generated macro for impl_116 (impl)
macro_rules! Depcrate_setimpl_116 {
() => {
// Module: crate::set
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a Set < T > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
