// Generated macro for impl_66 (impl)
macro_rules! Depcrate_walk_readdirimpl_66 {
() => {
// Module: crate::walk::readdir
// Provides: {"impl_66"}
// Dependencies: {}
impl Options < '_ > { fn should_hold (& self , status : entry :: Status) -> bool { if status . is_pruned () { return false ; } self . emit_ignored == Some (CollapseDirectory) || self . emit_untracked == CollapseDirectory } }
};
}
