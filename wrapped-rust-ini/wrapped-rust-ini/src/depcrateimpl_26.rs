// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl DoubleEndedIterator for PropertyIter < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (k , v) | (k . as_ref () , v . as_ref ())) } }
};
}
