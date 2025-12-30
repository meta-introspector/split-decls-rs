// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < K : Clone + Eq + Hash , V , S : BuildHasher , W : WeightScale < K , V > > DoubleEndedIterator for CLruCacheIntoIter < K , V , S , W > { fn next_back (& mut self) -> Option < Self :: Item > { self . cache . pop_back () } }
};
}
