// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_treeimpl_1287 {
() => {
// Module: crate::tree
// Provides: {"impl_1287"}
// Dependencies: {}
impl < 'repo > Clone for Tree < 'repo > { fn clone (& self) -> Self { self . as_object () . clone () . into_tree () . ok () . unwrap () } }
};
}
