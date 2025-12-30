// Generated macro for impl_152 (impl)
macro_rules! Depcrate_vecimpl_152 {
() => {
// Module: crate::vec
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'a , I : Idx , T > IntoIterator for & 'a IndexVec < I , T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; # [inline] fn into_iter (self) -> slice :: Iter < 'a , T > { self . iter () } }
};
}
