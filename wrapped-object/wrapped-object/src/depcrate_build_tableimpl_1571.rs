// Generated macro for impl_1571 (impl)
macro_rules! Depcrate_build_tableimpl_1571 {
() => {
// Module: crate::build::table
// Provides: {"impl_1571"}
// Dependencies: {}
impl < 'a , T : Item > Iterator for TableIter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { self . iter . find (| item | ! item . is_deleted ()) } }
};
}
