// Generated macro for impl_114 (impl)
macro_rules! Depcrateimpl_114 {
() => {
// Module: crate
// Provides: {"impl_114"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a CharClass { type Item = & 'a ClassRange ; type IntoIter = slice :: Iter < 'a , ClassRange > ; fn into_iter (self) -> slice :: Iter < 'a , ClassRange > { self . iter () } }
};
}
