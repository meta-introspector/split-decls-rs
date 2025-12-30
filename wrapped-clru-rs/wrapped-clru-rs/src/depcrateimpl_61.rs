// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl < K : Clone + Eq + Hash , V , S : BuildHasher , W : WeightScale < K , V > > Iterator for CLruCacheIntoIter < K , V , S , W > { type Item = (K , V) ; # [inline] fn next (& mut self) -> Option < (K , V) > { self . cache . pop_front () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . cache . len () , Some (self . cache . len ())) } }
};
}
