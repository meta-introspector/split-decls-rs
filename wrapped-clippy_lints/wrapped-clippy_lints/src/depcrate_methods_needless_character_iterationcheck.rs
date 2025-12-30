// Generated macro for check (function)
macro_rules! Depcrate_methods_needless_character_iterationcheck {
() => {
// Module: crate::methods::needless_character_iteration
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , call_expr : & Expr < '_ > , recv : & Expr < '_ > , closure_arg : & Expr < '_ > , is_all : bool) { if let ExprKind :: Closure (& Closure { body , .. }) = closure_arg . kind && let body = cx . tcx . hir_body (body) && let Some (first_param) = body . params . first () && let ExprKind :: MethodCall (method , mut recv , [] , _) = recv . kind && method . ident . name == sym :: chars && let str_ty = cx . typeck_results () . expr_ty_adjusted (recv) . peel_refs () && * str_ty . kind () == ty :: Str { let expr_start = recv . span ; while let ExprKind :: MethodCall (_ , new_recv , _ , _) = recv . kind { recv = new_recv ; } let body_expr = peel_blocks (body . value) ; handle_expr (cx , body_expr , first_param . pat . hir_id , recv . span . with_hi (call_expr . span . hi ()) , recv . span . with_hi (expr_start . hi ()) , false , is_all ,) ; } }
};
}
