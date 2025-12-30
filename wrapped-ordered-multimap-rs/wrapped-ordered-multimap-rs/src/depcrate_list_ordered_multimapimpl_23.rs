// Generated macro for impl_23 (impl)
macro_rules! Depcrate_list_ordered_multimapimpl_23 {
() => {
// Module: crate::list_ordered_multimap
// Provides: {"impl_23"}
// Dependencies: {}
impl < Key , Value , State > IntoIterator for ListOrderedMultimap < Key , Value , State > where Key : Clone , { type IntoIter = IntoIter < Key , Value > ; type Item = (Key , Value) ; fn into_iter (self) -> Self :: IntoIter { IntoIter { keys : self . keys , iter : self . values . into_iter () , } } }
};
}
