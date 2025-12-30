// Generated macro for impl_73 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_73 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_73"}
// Dependencies: {}
impl < Key , Value > Iterator for IntoIter < Key , Value > where Key : Clone , { type Item = (Key , Value) ; fn next (& mut self) -> Option < Self :: Item > { let value_entry = self . iter . next () ? ; let key = self . keys . get (value_entry . key_index) . cloned () . unwrap () ; Some ((key , value_entry . value)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
