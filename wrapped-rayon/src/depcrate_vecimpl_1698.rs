// Generated macro for impl_1698 (impl)
macro_rules! Depcrate_vecimpl_1698 {
() => {
// Module: crate::vec
// Provides: {"impl_1698"}
// Dependencies: {}
impl < T : Send > IntoParallelIterator for Vec < T > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { vec : self } } }
};
}
