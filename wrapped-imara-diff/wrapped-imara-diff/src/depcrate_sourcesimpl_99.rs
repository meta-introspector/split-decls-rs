// Generated macro for impl_99 (impl)
macro_rules! Depcrate_sourcesimpl_99 {
() => {
// Module: crate::sources
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > Iterator for Lines < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| it | unsafe { from_utf8_unchecked (it) }) } }
};
}
