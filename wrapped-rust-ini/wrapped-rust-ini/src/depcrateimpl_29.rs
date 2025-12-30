// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl DoubleEndedIterator for PropertyIterMut < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (k , v) | (k . as_ref () , v)) } }
};
}
