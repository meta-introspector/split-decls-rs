// Generated macro for impl_101 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_101 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; # [inline] fn next (& mut self) -> Option < & 'a V > { self . inner . next () . map (| e | e . 1) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
