// Generated macro for impl_157 (impl)
macro_rules! Depcrate_index_mapimpl_157 {
() => {
// Module: crate::index_map
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a , K , V > Iterator for ValuesMut < 'a , K , V > { type Item = & 'a mut V ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | & mut bucket . value) } }
};
}
