// Generated macro for impl_73 (impl)
macro_rules! Depcrate_ordered_mapimpl_73 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; fn next (& mut self) -> Option < & 'a K > { self . iter . next () . map (| e | e . 0) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
