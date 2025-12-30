// Generated macro for impl_148 (impl)
macro_rules! Depcrate_index_mapimpl_148 {
() => {
// Module: crate::index_map
// Provides: {"impl_148"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | (& bucket . key , & bucket . value)) } }
};
}
