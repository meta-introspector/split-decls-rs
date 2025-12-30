// Generated macro for impl_44 (impl)
macro_rules! Depcrate_mapimpl_44 {
() => {
// Module: crate::map
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; fn next (& mut self) -> Option < & 'a V > { self . iter . next () . map (| e | e . 1) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
