// Generated macro for impl_155 (impl)
macro_rules! Depcrate_index_mapimpl_155 {
() => {
// Module: crate::index_map
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | & bucket . value) } }
};
}
