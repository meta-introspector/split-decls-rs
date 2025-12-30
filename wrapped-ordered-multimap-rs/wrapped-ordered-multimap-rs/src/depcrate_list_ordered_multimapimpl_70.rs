// Generated macro for impl_70 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_70 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_70"}
// Dependencies: {}
impl < Key , Value > DoubleEndedIterator for IntoIter < Key , Value > where Key : Clone , { fn next_back (& mut self) -> Option < Self :: Item > { let value_entry = self . iter . next_back () ? ; let key = self . keys . get (value_entry . key_index) . cloned () . unwrap () ; Some ((key , value_entry . value)) } }
};
}
