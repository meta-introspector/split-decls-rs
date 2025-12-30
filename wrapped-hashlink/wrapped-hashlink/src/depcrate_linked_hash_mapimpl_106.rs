// Generated macro for impl_106 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_106 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a , K , V > Iterator for ValuesMut < 'a , K , V > { type Item = & 'a mut V ; # [inline] fn next (& mut self) -> Option < & 'a mut V > { self . inner . next () . map (| e | e . 1) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
