// Generated macro for impl_166 (impl)
macro_rules! Depcrate_linked_hash_setimpl_166 {
() => {
// Module: crate::linked_hash_set
// Provides: {"impl_166"}
// Dependencies: {}
impl < T , S > IntoIterator for LinkedHashSet < T , S > { type Item = T ; type IntoIter = IntoIter < T > ; # [inline] fn into_iter (self) -> IntoIter < T > { IntoIter { iter : self . map . into_iter () , } } }
};
}
