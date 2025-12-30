// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl < T > DoubleEndedIterator for Indices < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . tail . map (| index | { let entry = self . entries [index . get ()] . occupied_ref () ; let index = Index :: new (index , entry . generation) ; self . tail = entry . previous ; self . remaining -= 1 ; index }) } } }
};
}
