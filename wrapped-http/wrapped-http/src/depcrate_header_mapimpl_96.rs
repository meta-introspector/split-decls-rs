// Generated macro for impl_96 (impl)
macro_rules! Depcrate_header_mapimpl_96 {
() => {
// Module: crate::header::map
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a , 'b : 'a , T > IntoIterator for & 'b GetAll < 'a , T > { type Item = & 'a T ; type IntoIter = ValueIter < 'a , T > ; fn into_iter (self) -> ValueIter < 'a , T > { self . map . value_iter (self . index) } }
};
}
