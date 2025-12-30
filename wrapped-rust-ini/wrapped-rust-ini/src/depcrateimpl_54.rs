// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl DoubleEndedIterator for SectionIter < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (k , v) | (k . as_ref () . map (| s | s . as_str ()) , v)) } }
};
}
