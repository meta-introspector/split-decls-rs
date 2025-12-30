// Generated macro for impl_260 (impl)
macro_rules! Depcrateimpl_260 {
() => {
// Module: crate
// Provides: {"impl_260"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V , S : BuildHasher + Clone > IntoIterator for & 'a DashMap < K , V , S > { type Item = RefMulti < 'a , K , V > ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
