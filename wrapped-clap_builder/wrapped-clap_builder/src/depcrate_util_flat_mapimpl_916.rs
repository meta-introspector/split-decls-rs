// Generated macro for impl_916 (impl)
macro_rules! Depcrate_util_flat_mapimpl_916 {
() => {
// Module: crate::util::flat_map
// Provides: {"impl_916"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Iter < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a V) > { match self . keys . next_back () { Some (k) => { let v = self . values . next_back () . unwrap () ; Some ((k , v)) } None => None , } } }
};
}
