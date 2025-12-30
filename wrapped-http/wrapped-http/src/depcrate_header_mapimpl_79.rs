// Generated macro for impl_79 (impl)
macro_rules! Depcrate_header_mapimpl_79 {
() => {
// Module: crate::header::map
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a , T > Iterator for Keys < 'a , T > { type Item = & 'a HeaderName ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| b | & b . key) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . inner . nth (n) . map (| b | & b . key) } fn count (self) -> usize { self . inner . count () } fn last (self) -> Option < Self :: Item > { self . inner . last () . map (| b | & b . key) } }
};
}
