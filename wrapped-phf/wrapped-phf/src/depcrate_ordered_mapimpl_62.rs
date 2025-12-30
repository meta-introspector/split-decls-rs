// Generated macro for impl_62 (impl)
macro_rules! Depcrate_ordered_mapimpl_62 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a , K , V > IntoIterator for & 'a OrderedMap < K , V > { type Item = (& 'a K , & 'a V) ; type IntoIter = Entries < 'a , K , V > ; fn into_iter (self) -> Entries < 'a , K , V > { self . entries () } }
};
}
