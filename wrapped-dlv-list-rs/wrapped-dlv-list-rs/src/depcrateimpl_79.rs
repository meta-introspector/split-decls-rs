// Generated macro for impl_79 (impl)
macro_rules! Depcrateimpl_79 {
() => {
// Module: crate
// Provides: {"impl_79"}
// Dependencies: {}
impl < T > DoubleEndedIterator for IntoIter < T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . tail . map (| index | { let entry = self . list . remove_entry (index) . expect ("expected occupied entry") ; self . tail = entry . previous ; self . remaining -= 1 ; entry . value }) } } }
};
}
