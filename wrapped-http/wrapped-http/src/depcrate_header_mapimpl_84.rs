// Generated macro for impl_84 (impl)
macro_rules! Depcrate_header_mapimpl_84 {
() => {
// Module: crate::header::map
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a , T > Iterator for ValuesMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (_ , v) | v) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
