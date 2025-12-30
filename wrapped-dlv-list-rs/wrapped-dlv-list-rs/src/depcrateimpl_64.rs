// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < T > DoubleEndedIterator for Drain < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . tail . map (| index | { let entry = self . list . remove_entry (index) . expect ("expected occupied entry") ; self . tail = entry . previous ; self . remaining -= 1 ; entry . value }) } } }
};
}
