// Generated macro for impl_205 (impl)
macro_rules! Depcrate_ord_mapimpl_205 {
() => {
// Module: crate::ord::map
// Provides: {"impl_205"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Keys < 'a , K , V > where K : 'a + Ord , V : 'a , { type Item = & 'a K ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| (k , _) | k) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
