// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'a , K , V , S , W : WeightScale < K , V > > IntoIterator for & 'a CLruCache < K , V , S , W > { type Item = (& 'a K , & 'a V) ; type IntoIter = CLruCacheIter < 'a , K , V > ; # [inline] fn into_iter (self) -> CLruCacheIter < 'a , K , V > { self . iter () } }
};
}
