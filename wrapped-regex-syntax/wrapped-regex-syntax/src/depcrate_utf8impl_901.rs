// Generated macro for impl_901 (impl)
macro_rules! Depcrate_utf8impl_901 {
() => {
// Module: crate::utf8
// Provides: {"impl_901"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Utf8Sequence { type IntoIter = slice :: Iter < 'a , Utf8Range > ; type Item = & 'a Utf8Range ; fn into_iter (self) -> Self :: IntoIter { self . as_slice () . iter () } }
};
}
