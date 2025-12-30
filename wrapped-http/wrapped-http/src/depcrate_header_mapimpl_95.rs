// Generated macro for impl_95 (impl)
macro_rules! Depcrate_header_mapimpl_95 {
() => {
// Module: crate::header::map
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a , T > IntoIterator for GetAll < 'a , T > { type Item = & 'a T ; type IntoIter = ValueIter < 'a , T > ; fn into_iter (self) -> ValueIter < 'a , T > { self . map . value_iter (self . index) } }
};
}
