// Generated macro for impl_353 (impl)
macro_rules! Depcrate_hash_mapimpl_353 {
() => {
// Module: crate::hash::map
// Provides: {"impl_353"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| ((k , _) , _) | k) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
