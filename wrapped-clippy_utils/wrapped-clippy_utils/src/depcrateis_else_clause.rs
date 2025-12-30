// Generated macro for is_else_clause (function)
macro_rules! Depcrateis_else_clause {
() => {
// Module: crate
// Provides: {"is_else_clause"}
// Dependencies: {}
# [doc = " Checks if the given expression is the else clause of either an `if` or `if let` expression."] pub fn is_else_clause (tcx : TyCtxt < '_ > , expr : & Expr < '_ >) -> bool { let mut iter = tcx . hir_parent_iter (expr . hir_id) ; match iter . next () { Some ((_ , Node :: Expr (Expr { kind : ExprKind :: If (_ , _ , Some (else_expr)) , .. }) ,)) => else_expr . hir_id == expr . hir_id , _ => false , } }
};
}
