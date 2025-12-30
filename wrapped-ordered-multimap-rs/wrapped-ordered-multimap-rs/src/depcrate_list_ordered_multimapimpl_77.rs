// Generated macro for impl_77 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_77 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_77"}
// Dependencies: {}
impl < Key , Value > DoubleEndedIterator for Iter < '_ , Key , Value > { fn next_back (& mut self) -> Option < Self :: Item > { let value_entry = self . iter . next_back () ? ; let key = self . keys . get (value_entry . key_index) . unwrap () ; Some ((key , & value_entry . value)) } }
};
}
