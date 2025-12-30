// Generated macro for extract_set_len_self (function)
macro_rules! Depcrate_uninit_vecextract_set_len_self {
() => {
// Module: crate::uninit_vec
// Provides: {"extract_set_len_self"}
// Dependencies: {}
# [doc = " Returns self if the expression is `Vec::set_len()`"] fn extract_set_len_self < 'tcx > (cx : & LateContext < '_ > , expr : & 'tcx Expr < '_ >) -> Option < (& 'tcx Expr < 'tcx > , Span) > { let expr = peel_hir_expr_while (expr , | e | { if let ExprKind :: Block (block , _) = e . kind { match (block . stmts . first () . map (| stmt | & stmt . kind) , block . expr) { (None , Some (expr)) => Some (expr) , (Some (StmtKind :: Expr (expr) | StmtKind :: Semi (expr)) , _) => Some (expr) , _ => None , } } else { None } }) ; match expr . kind { ExprKind :: MethodCall (path , self_expr , [arg] , _) => { let self_type = cx . typeck_results () . expr_ty (self_expr) . peel_refs () ; if self_type . is_diag_item (cx , sym :: Vec) && path . ident . name == sym :: set_len && ! is_integer_literal (arg , 0) { Some ((self_expr , expr . span)) } else { None } } , _ => None , } }
};
}
