// Generated macro for impl_341 (impl)
macro_rules! Depcrate_hash_mapimpl_341 {
() => {
// Module: crate::hash::map
// Provides: {"impl_341"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| ((k , v) , _) | (k , v)) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
