// Generated macro for impl_151 (impl)
macro_rules! Depcrate_vecimpl_151 {
() => {
// Module: crate::vec
// Provides: {"impl_151"}
// Dependencies: {}
impl < I : Idx , T > IntoIterator for IndexVec < I , T > { type Item = T ; type IntoIter = vec :: IntoIter < T > ; # [inline] fn into_iter (self) -> vec :: IntoIter < T > { self . raw . into_iter () } }
};
}
