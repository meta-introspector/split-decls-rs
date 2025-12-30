// Generated macro for check_expr (function)
macro_rules! Depcrate_unused_io_amountcheck_expr {
() => {
// Module: crate::unused_io_amount
// Provides: {"check_expr"}
// Dependencies: {}
fn check_expr < 'a > (cx : & LateContext < 'a > , expr : & 'a hir :: Expr < 'a >) { match expr . kind { ExprKind :: If (cond , _ , _) if let ExprKind :: Let (hir :: LetExpr { pat , init , .. }) = cond . kind && is_ok_wild_or_dotdot_pattern (cx , pat) && let Some (op) = should_lint (cx , init) => { emit_lint (cx , cond . span , cond . hir_id , op , & [pat . span]) ; } , ExprKind :: Match (expr , [arm1 , arm2] , hir :: MatchSource :: Normal) if let Some (op) = should_lint (cx , expr) => { if non_consuming_ok_arm (cx , arm1) && non_consuming_err_arm (cx , arm2) { emit_lint (cx , expr . span , expr . hir_id , op , & [arm1 . pat . span]) ; } if non_consuming_ok_arm (cx , arm2) && non_consuming_err_arm (cx , arm1) { emit_lint (cx , expr . span , expr . hir_id , op , & [arm2 . pat . span]) ; } } , ExprKind :: Match (_ , _ , hir :: MatchSource :: Normal) => { } , _ if let Some (op) = should_lint (cx , expr) => { emit_lint (cx , expr . span , expr . hir_id , op , & []) ; } , _ => { } , } }
};
}
