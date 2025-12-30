// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < K : Clone + Eq + Hash , V , S : BuildHasher , W : WeightScale < K , V > > IntoIterator for CLruCache < K , V , S , W > { type Item = (K , V) ; type IntoIter = CLruCacheIntoIter < K , V , S , W > ; # [doc = " Consumes the cache into an iterator yielding elements by value."] # [inline] fn into_iter (self) -> CLruCacheIntoIter < K , V , S , W > { CLruCacheIntoIter { cache : self } } }
};
}
