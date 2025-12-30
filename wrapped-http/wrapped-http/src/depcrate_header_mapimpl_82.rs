// Generated macro for impl_82 (impl)
macro_rules! Depcrate_header_mapimpl_82 {
() => {
// Module: crate::header::map
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a , T > Iterator for Values < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (_ , v) | v) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
