// Generated macro for impl_44 (impl)
macro_rules! Depcrate_range_set_btree_range_setimpl_44 {
() => {
// Module: crate::range_set::btree_range_set
// Provides: {"impl_44"}
// Dependencies: {}
impl Iterator for Iter < '_ > { type Item = Range < u64 > ; fn next (& mut self) -> Option < Range < u64 > > { let (& start , & end) = self . 0 . next () ? ; Some (start .. end) } }
};
}
