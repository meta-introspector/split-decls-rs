// Generated macro for impl_1699 (impl)
macro_rules! Depcrate_vecimpl_1699 {
() => {
// Module: crate::vec
// Provides: {"impl_1699"}
// Dependencies: {}
impl < T : Send > IntoParallelIterator for Box < [T] > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { vec : self . into () } } }
};
}
