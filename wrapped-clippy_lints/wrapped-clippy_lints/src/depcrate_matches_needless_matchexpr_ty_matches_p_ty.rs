// Generated macro for expr_ty_matches_p_ty (function)
macro_rules! Depcrate_matches_needless_matchexpr_ty_matches_p_ty {
() => {
// Module: crate::matches::needless_match
// Provides: {"expr_ty_matches_p_ty"}
// Dependencies: {}
# [doc = " Manually check for coercion casting by checking if the type of the match operand or let expr"] # [doc = " differs with the assigned local variable or the function return type."] fn expr_ty_matches_p_ty (cx : & LateContext < '_ > , expr : & Expr < '_ > , p_expr : & Expr < '_ >) -> bool { match cx . tcx . parent_hir_node (p_expr . hir_id) { Node :: LetStmt (local) => { let results = cx . typeck_results () ; return same_type_modulo_regions (results . node_type (local . hir_id) , results . expr_ty (expr)) ; } , Node :: Item (item) => { if let ItemKind :: Fn { .. } = item . kind { let output = cx . tcx . fn_sig (item . owner_id) . instantiate_identity () . output () . skip_binder () ; return same_type_modulo_regions (output , cx . typeck_results () . expr_ty (expr)) ; } } , Node :: Block (block) => { if let Some (block_parent_expr) = get_parent_expr_for_hir (cx , block . hir_id) { return expr_ty_matches_p_ty (cx , expr , block_parent_expr) ; } } , Node :: Expr (p_expr) => { return expr_ty_matches_p_ty (cx , expr , p_expr) ; } , _ => { } , } false }
};
}
