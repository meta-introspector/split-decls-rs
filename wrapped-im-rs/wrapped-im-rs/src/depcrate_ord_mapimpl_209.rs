// Generated macro for impl_209 (impl)
macro_rules! Depcrate_ord_mapimpl_209 {
() => {
// Module: crate::ord::map
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Values < 'a , K , V > where K : 'a + Ord , V : 'a , { type Item = & 'a V ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| (_ , v) | v) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
