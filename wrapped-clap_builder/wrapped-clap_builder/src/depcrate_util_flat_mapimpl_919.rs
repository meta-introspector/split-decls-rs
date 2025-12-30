// Generated macro for impl_919 (impl)
macro_rules! Depcrate_util_flat_mapimpl_919 {
() => {
// Module: crate::util::flat_map
// Provides: {"impl_919"}
// Dependencies: {}
impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { match self . keys . next () { Some (k) => { let v = self . values . next () . unwrap () ; Some ((k , v)) } None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { self . keys . size_hint () } }
};
}
