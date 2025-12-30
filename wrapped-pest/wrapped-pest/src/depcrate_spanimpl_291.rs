// Generated macro for impl_291 (impl)
macro_rules! Depcrate_spanimpl_291 {
() => {
// Module: crate::span
// Provides: {"impl_291"}
// Dependencies: {}
impl < 'i > Iterator for Lines < 'i > { type Item = & 'i str ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| span | span . as_str ()) } }
};
}
