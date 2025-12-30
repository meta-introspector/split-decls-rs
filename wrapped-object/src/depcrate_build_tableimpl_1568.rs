// Generated macro for impl_1568 (impl)
macro_rules! Depcrate_build_tableimpl_1568 {
() => {
// Module: crate::build::table
// Provides: {"impl_1568"}
// Dependencies: {}
impl < 'a , T : Item > IntoIterator for & 'a Table < T > { type Item = & 'a T ; type IntoIter = TableIter < 'a , T > ; fn into_iter (self) -> TableIter < 'a , T > { TableIter { iter : self . 0 . iter () , } } }
};
}
