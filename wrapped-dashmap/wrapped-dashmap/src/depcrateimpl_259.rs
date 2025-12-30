// Generated macro for impl_259 (impl)
macro_rules! Depcrateimpl_259 {
() => {
// Module: crate
// Provides: {"impl_259"}
// Dependencies: {}
impl < K : Eq + Hash , V , S : BuildHasher + Clone > IntoIterator for DashMap < K , V , S > { type Item = (K , V) ; type IntoIter = OwningIter < K , V > ; fn into_iter (self) -> Self :: IntoIter { OwningIter :: new (self) } }
};
}
