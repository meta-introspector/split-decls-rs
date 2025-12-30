// Generated macro for impl_141 (impl)
macro_rules! Depcrate_index_mapimpl_141 {
() => {
// Module: crate::index_map
// Provides: {"impl_141"}
// Dependencies: {}
impl < K , V , const N : usize > Iterator for IntoIter < K , V , N > { type Item = (K , V) ; fn next (& mut self) -> Option < Self :: Item > { self . entries . pop () . map (| bucket | (bucket . key , bucket . value)) } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
