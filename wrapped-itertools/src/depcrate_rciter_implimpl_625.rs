// Generated macro for impl_625 (impl)
macro_rules! Depcrate_rciter_implimpl_625 {
() => {
// Module: crate::rciter_impl
// Provides: {"impl_625"}
// Dependencies: {}
impl < I > DoubleEndedIterator for RcIter < I > where I : DoubleEndedIterator , { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . rciter . borrow_mut () . next_back () } }
};
}
