// Generated macro for impl_48 (impl)
macro_rules! Depcrate_range_set_btree_range_setimpl_48 {
() => {
// Module: crate::range_set::btree_range_set
// Provides: {"impl_48"}
// Dependencies: {}
impl Iterator for EltIter < '_ > { type Item = u64 ; fn next (& mut self) -> Option < u64 > { if self . next == self . end { let (& start , & end) = self . inner . next () ? ; self . next = start ; self . end = end ; } let x = self . next ; self . next += 1 ; Some (x) } }
};
}
