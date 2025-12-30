// Generated macro for impl_84 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_84 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_84"}
// Dependencies: {}
impl < Key , Value > DoubleEndedIterator for IterMut < '_ , Key , Value > { fn next_back (& mut self) -> Option < Self :: Item > { let value_entry = self . iter . next_back () ? ; let key = self . keys . get (value_entry . key_index) . unwrap () ; Some ((key , & mut value_entry . value)) } }
};
}
