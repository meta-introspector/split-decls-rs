// Generated macro for impl_268 (impl)
macro_rules! Depcrate_ord_setimpl_268 {
() => {
// Module: crate::ord::set
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'a , A > DoubleEndedIterator for Iter < 'a , A > where A : 'a + Ord , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (Deref :: deref) } }
};
}
