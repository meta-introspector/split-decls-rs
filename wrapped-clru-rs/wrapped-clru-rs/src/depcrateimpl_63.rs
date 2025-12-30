// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < K : Clone + Eq + Hash , V , S : BuildHasher , W : WeightScale < K , V > > ExactSizeIterator for CLruCacheIntoIter < K , V , S , W > { fn len (& self) -> usize { self . size_hint () . 0 } }
};
}
