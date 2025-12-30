// Generated macro for impl_1573 (impl)
macro_rules! Depcrate_build_tableimpl_1573 {
() => {
// Module: crate::build::table
// Provides: {"impl_1573"}
// Dependencies: {}
impl < 'a , T : Item > Iterator for TableIterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < & 'a mut T > { self . iter . find (| item | ! item . is_deleted ()) } }
};
}
