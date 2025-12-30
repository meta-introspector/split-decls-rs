// Generated macro for impl_49 (impl)
macro_rules! Depcrate_range_set_btree_range_setimpl_49 {
() => {
// Module: crate::range_set::btree_range_set
// Provides: {"impl_49"}
// Dependencies: {}
impl DoubleEndedIterator for EltIter < '_ > { fn next_back (& mut self) -> Option < u64 > { if self . next == self . end { let (& start , & end) = self . inner . next_back () ? ; self . next = start ; self . end = end ; } self . end -= 1 ; Some (self . end) } }
};
}
