// Generated macro for impl_80 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_80 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'map , Key , Value > Iterator for Iter < 'map , Key , Value > { type Item = (& 'map Key , & 'map Value) ; fn next (& mut self) -> Option < Self :: Item > { let value_entry = self . iter . next () ? ; let key = self . keys . get (value_entry . key_index) . unwrap () ; Some ((key , & value_entry . value)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
