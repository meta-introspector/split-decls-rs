// Generated macro for impl_1289 (impl)
macro_rules! Depcrate_treeimpl_1289 {
() => {
// Module: crate::tree
// Provides: {"impl_1289"}
// Dependencies: {}
impl < 'repo , 'iter > IntoIterator for & 'iter Tree < 'repo > { type Item = TreeEntry < 'iter > ; type IntoIter = TreeIter < 'iter > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
