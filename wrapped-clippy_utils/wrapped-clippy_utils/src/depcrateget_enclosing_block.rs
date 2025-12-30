// Generated macro for get_enclosing_block (function)
macro_rules! Depcrateget_enclosing_block {
() => {
// Module: crate
// Provides: {"get_enclosing_block"}
// Dependencies: {}
# [doc = " Gets the enclosing block, if any."] pub fn get_enclosing_block < 'tcx > (cx : & LateContext < 'tcx > , hir_id : HirId) -> Option < & 'tcx Block < 'tcx > > { let enclosing_node = cx . tcx . hir_get_enclosing_scope (hir_id) . map (| enclosing_id | cx . tcx . hir_node (enclosing_id)) ; enclosing_node . and_then (| node | match node { Node :: Block (block) => Some (block) , Node :: Item (& Item { kind : ItemKind :: Fn { body : eid , .. } , .. }) | Node :: ImplItem (& ImplItem { kind : ImplItemKind :: Fn (_ , eid) , .. }) | Node :: TraitItem (& TraitItem { kind : TraitItemKind :: Fn (_ , TraitFn :: Provided (eid)) , .. }) => match cx . tcx . hir_body (eid) . value . kind { ExprKind :: Block (block , _) => Some (block) , _ => None , } , _ => None , }) }
};
}
