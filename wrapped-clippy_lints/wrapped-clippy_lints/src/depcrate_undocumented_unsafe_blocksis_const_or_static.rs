// Generated macro for is_const_or_static (function)
macro_rules! Depcrate_undocumented_unsafe_blocksis_const_or_static {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"is_const_or_static"}
// Dependencies: {}
fn is_const_or_static (node : & Node < '_ >) -> bool { matches ! (node , Node :: Item (hir :: Item { kind : ItemKind :: Const (..) | ItemKind :: Static (..) , .. }) | Node :: ImplItem (hir :: ImplItem { kind : hir :: ImplItemKind :: Const (..) , .. }) | Node :: TraitItem (hir :: TraitItem { kind : hir :: TraitItemKind :: Const (..) , .. })) }
};
}
