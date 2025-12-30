// Generated macro for impl_66 (impl)
macro_rules! Depcrate_ordered_mapimpl_66 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Entries < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < (& 'a K , & 'a V) > { self . iter . next () . map (| e | (& e . 0 , & e . 1)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
