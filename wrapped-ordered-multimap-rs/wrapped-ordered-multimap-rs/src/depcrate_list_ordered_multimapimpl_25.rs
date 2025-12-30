// Generated macro for impl_25 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_25 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'map , Key , Value , State > IntoIterator for & 'map mut ListOrderedMultimap < Key , Value , State > { type IntoIter = IterMut < 'map , Key , Value > ; type Item = (& 'map Key , & 'map mut Value) ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
