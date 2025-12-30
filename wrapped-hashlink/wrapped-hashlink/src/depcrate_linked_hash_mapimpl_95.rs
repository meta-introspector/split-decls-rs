// Generated macro for impl_95 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_95 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; # [inline] fn next (& mut self) -> Option < & 'a K > { self . inner . next () . map (| e | e . 0) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
