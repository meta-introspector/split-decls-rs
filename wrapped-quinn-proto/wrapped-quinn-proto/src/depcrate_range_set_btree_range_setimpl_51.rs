// Generated macro for impl_51 (impl)
macro_rules! Depcrate_range_set_btree_range_setimpl_51 {
() => {
// Module: crate::range_set::btree_range_set
// Provides: {"impl_51"}
// Dependencies: {}
impl Iterator for Replace < '_ > { type Item = Range < u64 > ; fn next (& mut self) -> Option < Range < u64 > > { if let Some (pred) = self . pred . take () { return Some (pred) ; } let (next_start , next_end) = self . set . succ (self . range . start) ? ; if next_start > self . range . end { return None ; } self . set . 0 . remove (& next_start) ; let replaced_end = self . range . end . min (next_end) ; self . range . end = self . range . end . max (next_end) ; if next_start == replaced_end { None } else { Some (next_start .. replaced_end) } } }
};
}
