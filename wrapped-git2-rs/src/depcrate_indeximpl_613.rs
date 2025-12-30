// Generated macro for impl_613 (impl)
macro_rules! Depcrate_indeximpl_613 {
() => {
// Module: crate::index
// Provides: {"impl_613"}
// Dependencies: {}
impl < 'index > Iterator for IndexEntries < 'index > { type Item = IndexEntry ; fn next (& mut self) -> Option < IndexEntry > { self . range . next () . map (| i | self . index . get (i) . unwrap ()) } }
};
}
