// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < K , V , S , W : WeightScale < K , V > > CLruCache < K , V , S , W > { # [doc = " Returns an iterator visiting all entries in order."] # [doc = " The iterator element type is `(&'a K, &'a V)`."] pub fn iter (& self) -> CLruCacheIter < '_ , K , V > { CLruCacheIter { iter : self . storage . iter () , } } }
};
}
