// Generated macro for impl_357 (impl)
macro_rules! Depcrate_hash_mapimpl_357 {
() => {
// Module: crate::hash::map
// Provides: {"impl_357"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| ((_ , v) , _) | v) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
