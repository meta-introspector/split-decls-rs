// Generated macro for impl_920 (impl)
macro_rules! Depcrate_util_flat_mapimpl_920 {
() => {
// Module: crate::util::flat_map
// Provides: {"impl_920"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for IterMut < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a mut V) > { match self . keys . next_back () { Some (k) => { let v = self . values . next_back () . unwrap () ; Some ((k , v)) } None => None , } } }
};
}
