// Generated macro for check_last_stmt_in_expr (function)
macro_rules! Depcrate_needless_continuecheck_last_stmt_in_expr {
() => {
// Module: crate::needless_continue
// Provides: {"check_last_stmt_in_expr"}
// Dependencies: {}
fn check_last_stmt_in_expr < F > (cx : & LateContext < '_ > , inner_expr : & Expr < '_ > , func : & F) where F : Fn (Option < & Label > , Span) , { match inner_expr . kind { ExprKind :: Continue (continue_label) => { func (continue_label . label . as_ref () , inner_expr . span) ; } , ExprKind :: If (_ , then_block , else_block) if let ExprKind :: Block (then_block , _) = then_block . kind => { check_last_stmt_in_block (cx , then_block , func) ; if let Some (else_block) = else_block { check_last_stmt_in_expr (cx , else_block , func) ; } } , ExprKind :: Match (_ , arms , _) => { let match_ty = cx . typeck_results () . expr_ty (inner_expr) ; if ! match_ty . is_unit () && ! match_ty . is_never () { return ; } for arm in arms { check_last_stmt_in_expr (cx , arm . body , func) ; } } , ExprKind :: Block (b , _) => { check_last_stmt_in_block (cx , b , func) ; } , _ => { } , } }
};
}
