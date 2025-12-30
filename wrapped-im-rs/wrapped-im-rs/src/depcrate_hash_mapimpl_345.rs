// Generated macro for impl_345 (impl)
macro_rules! Depcrate_hash_mapimpl_345 {
() => {
// Module: crate::hash::map
// Provides: {"impl_345"}
// Dependencies: {}
impl < 'a , K , V > Iterator for IterMut < 'a , K , V > where K : Clone , V : Clone , { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| ((k , v) , _) | (& * k , v)) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
