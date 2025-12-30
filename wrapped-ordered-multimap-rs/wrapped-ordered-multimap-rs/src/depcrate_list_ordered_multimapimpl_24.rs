// Generated macro for impl_24 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_24 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'map , Key , Value , State > IntoIterator for & 'map ListOrderedMultimap < Key , Value , State > { type IntoIter = Iter < 'map , Key , Value > ; type Item = (& 'map Key , & 'map Value) ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
