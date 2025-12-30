// Generated macro for impl_109 (impl)
macro_rules! Depcrate_header_mapimpl_109 {
() => {
// Module: crate::header::map
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a , T > IntoIterator for OccupiedEntry < 'a , T > { type Item = & 'a mut T ; type IntoIter = ValueIterMut < 'a , T > ; fn into_iter (self) -> ValueIterMut < 'a , T > { self . map . value_iter_mut (self . index) } }
};
}
