// Generated macro for impl_87 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_87 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'map , Key , Value > Iterator for IterMut < 'map , Key , Value > { type Item = (& 'map Key , & 'map mut Value) ; fn next (& mut self) -> Option < Self :: Item > { let value_entry = self . iter . next () ? ; let key = self . keys . get (value_entry . key_index) . unwrap () ; Some ((key , & mut value_entry . value)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
