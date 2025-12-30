// Generated macro for impl_873 (impl)
macro_rules! Depcrate_unordimpl_873 {
() => {
// Module: crate::unord
// Provides: {"impl_873"}
// Dependencies: {}
impl < K : Hash + Eq , V > FromIterator < (K , V) > for UnordMap < K , V > { # [inline] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { UnordMap { inner : FxHashMap :: from_iter (iter) } } }
};
}
