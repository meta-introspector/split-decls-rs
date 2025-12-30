// Generated macro for impl_219 (impl)
macro_rules! Depcrate_linear_mapimpl_219 {
() => {
// Module: crate::linear_map
// Provides: {"impl_219"}
// Dependencies: {}
impl < 'a , K , V , S : LinearMapStorage < K , V > + ? Sized > IntoIterator for & 'a LinearMapInner < K , V , S > where K : Eq , { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
