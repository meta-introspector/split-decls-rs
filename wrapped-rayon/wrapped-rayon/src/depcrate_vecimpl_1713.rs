// Generated macro for impl_1713 (impl)
macro_rules! Depcrate_vecimpl_1713 {
() => {
// Module: crate::vec
// Provides: {"impl_1713"}
// Dependencies: {}
impl < 'data , T : 'data > DoubleEndedIterator for SliceDrain < 'data , T > { fn next_back (& mut self) -> Option < Self :: Item > { let ptr : * const T = self . iter . next_back () ? ; Some (unsafe { ptr :: read (ptr) }) } }
};
}
