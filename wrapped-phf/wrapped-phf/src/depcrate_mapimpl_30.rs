// Generated macro for impl_30 (impl)
macro_rules! Depcrate_mapimpl_30 {
() => {
// Module: crate::map
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Entries < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < (& 'a K , & 'a V) > { self . iter . next () . map (| (k , v) | (k , v)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
