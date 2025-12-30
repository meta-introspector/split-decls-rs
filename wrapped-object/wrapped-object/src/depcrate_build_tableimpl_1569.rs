// Generated macro for impl_1569 (impl)
macro_rules! Depcrate_build_tableimpl_1569 {
() => {
// Module: crate::build::table
// Provides: {"impl_1569"}
// Dependencies: {}
impl < 'a , T : Item > IntoIterator for & 'a mut Table < T > { type Item = & 'a mut T ; type IntoIter = TableIterMut < 'a , T > ; fn into_iter (self) -> TableIterMut < 'a , T > { TableIterMut { iter : self . 0 . iter_mut () , } } }
};
}
