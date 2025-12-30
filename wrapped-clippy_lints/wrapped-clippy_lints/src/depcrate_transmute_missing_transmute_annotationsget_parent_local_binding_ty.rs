// Generated macro for get_parent_local_binding_ty (function)
macro_rules! Depcrate_transmute_missing_transmute_annotationsget_parent_local_binding_ty {
() => {
// Module: crate::transmute::missing_transmute_annotations
// Provides: {"get_parent_local_binding_ty"}
// Dependencies: {}
fn get_parent_local_binding_ty < 'tcx > (cx : & LateContext < 'tcx > , expr_hir_id : HirId) -> Option < LetStmt < 'tcx > > { let mut parent_iter = cx . tcx . hir_parent_iter (expr_hir_id) ; if let Some ((_ , node)) = parent_iter . next () { match node { Node :: LetStmt (local) => Some (* local) , Node :: Block (_) => { if let Some ((parent_hir_id , Node :: Expr (expr))) = parent_iter . next () && matches ! (expr . kind , rustc_hir :: ExprKind :: Block (_ , _)) { get_parent_local_binding_ty (cx , parent_hir_id) } else { None } } , _ => None , } } else { None } }
};
}
