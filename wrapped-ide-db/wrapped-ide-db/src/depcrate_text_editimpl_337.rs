// Generated macro for impl_337 (impl)
macro_rules! Depcrate_text_editimpl_337 {
() => {
// Module: crate::text_edit
// Provides: {"impl_337"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a TextEdit { type Item = & 'a Indel ; type IntoIter = std :: slice :: Iter < 'a , Indel > ; fn into_iter (self) -> Self :: IntoIter { self . indels . iter () } }
};
}
