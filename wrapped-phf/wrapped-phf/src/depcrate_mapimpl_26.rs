// Generated macro for impl_26 (impl)
macro_rules! Depcrate_mapimpl_26 {
() => {
// Module: crate::map
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a , K , V > IntoIterator for & 'a Map < K , V > { type Item = (& 'a K , & 'a V) ; type IntoIter = Entries < 'a , K , V > ; fn into_iter (self) -> Entries < 'a , K , V > { self . entries () } }
};
}
