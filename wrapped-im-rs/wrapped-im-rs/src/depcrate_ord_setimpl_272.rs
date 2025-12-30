// Generated macro for impl_272 (impl)
macro_rules! Depcrate_ord_setimpl_272 {
() => {
// Module: crate::ord::set
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'a , A > DoubleEndedIterator for RangedIter < 'a , A > where A : 'a + Ord , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (Deref :: deref) } }
};
}
