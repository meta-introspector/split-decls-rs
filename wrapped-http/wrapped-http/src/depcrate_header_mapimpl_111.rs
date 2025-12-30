// Generated macro for impl_111 (impl)
macro_rules! Depcrate_header_mapimpl_111 {
() => {
// Module: crate::header::map
// Provides: {"impl_111"}
// Dependencies: {}
impl < 'a , 'b : 'a , T > IntoIterator for & 'b mut OccupiedEntry < 'a , T > { type Item = & 'a mut T ; type IntoIter = ValueIterMut < 'a , T > ; fn into_iter (self) -> ValueIterMut < 'a , T > { self . iter_mut () } }
};
}
