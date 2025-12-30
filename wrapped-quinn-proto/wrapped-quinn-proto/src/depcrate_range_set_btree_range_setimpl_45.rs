// Generated macro for impl_45 (impl)
macro_rules! Depcrate_range_set_btree_range_setimpl_45 {
() => {
// Module: crate::range_set::btree_range_set
// Provides: {"impl_45"}
// Dependencies: {}
impl DoubleEndedIterator for Iter < '_ > { fn next_back (& mut self) -> Option < Range < u64 > > { let (& start , & end) = self . 0 . next_back () ? ; Some (start .. end) } }
};
}
