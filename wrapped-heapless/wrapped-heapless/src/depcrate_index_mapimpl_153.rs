// Generated macro for impl_153 (impl)
macro_rules! Depcrate_index_mapimpl_153 {
() => {
// Module: crate::index_map
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | & bucket . key) } }
};
}
