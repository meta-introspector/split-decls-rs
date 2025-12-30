// Generated macro for impl_88 (impl)
macro_rules! Depcrate_mapimpl_88 {
() => {
// Module: crate::map
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'a , K , V > IntoIterator for & 'a SkipMap < K , V > where K : Ord , { type Item = Entry < 'a , K , V > ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
};
}
