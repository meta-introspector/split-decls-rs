// Generated macro for impl_915 (impl)
macro_rules! Depcrate_util_flat_mapimpl_915 {
() => {
// Module: crate::util::flat_map
// Provides: {"impl_915"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < (& 'a K , & 'a V) > { match self . keys . next () { Some (k) => { let v = self . values . next () . unwrap () ; Some ((k , v)) } None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { self . keys . size_hint () } }
};
}
