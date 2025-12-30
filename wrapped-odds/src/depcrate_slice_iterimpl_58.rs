// Generated macro for impl_58 (impl)
macro_rules! Depcrate_slice_iterimpl_58 {
() => {
// Module: crate::slice::iter
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , T > DoubleEndedIterator for SliceCopyIter < 'a , T > where T : Copy , { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { if self . ptr != self . end { unsafe { self . end = self . end . offset (- 1) ; let elt = Some (* self . end) ; elt } } else { None } } }
};
}
