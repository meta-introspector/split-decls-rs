// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl < T > DoubleEndedIterator for Iter < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . tail . map (| index | { let entry = self . entries [index . get ()] . occupied_ref () ; self . tail = entry . previous ; self . remaining -= 1 ; & entry . value }) } } }
};
}
