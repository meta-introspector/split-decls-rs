// Generated macro for impl_110 (impl)
macro_rules! Depcrate_header_mapimpl_110 {
() => {
// Module: crate::header::map
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'a , 'b : 'a , T > IntoIterator for & 'b OccupiedEntry < 'a , T > { type Item = & 'a T ; type IntoIter = ValueIter < 'a , T > ; fn into_iter (self) -> ValueIter < 'a , T > { self . iter () } }
};
}
