// Generated macro for parent_item_name (function)
macro_rules! Depcrateparent_item_name {
() => {
// Module: crate
// Provides: {"parent_item_name"}
// Dependencies: {}
# [doc = " Gets the name of the item the expression is in, if available."] pub fn parent_item_name (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < Symbol > { let parent_id = cx . tcx . hir_get_parent_item (expr . hir_id) . def_id ; match cx . tcx . hir_node_by_def_id (parent_id) { Node :: Item (item) => item . kind . ident () . map (| ident | ident . name) , Node :: TraitItem (TraitItem { ident , .. }) | Node :: ImplItem (ImplItem { ident , .. }) => Some (ident . name) , _ => None , } }
};
}
