// Generated macro for impl_151 (impl)
macro_rules! Depcrate_index_mapimpl_151 {
() => {
// Module: crate::index_map
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | (& bucket . key , & mut bucket . value)) } }
};
}
