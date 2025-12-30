// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl DoubleEndedIterator for SectionIterMut < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (k , v) | (k . as_ref () . map (| s | s . as_str ()) , v)) } }
};
}
