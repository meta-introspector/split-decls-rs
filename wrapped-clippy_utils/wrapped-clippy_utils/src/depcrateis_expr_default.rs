// Generated macro for is_expr_default (function)
macro_rules! Depcrateis_expr_default {
() => {
// Module: crate
// Provides: {"is_expr_default"}
// Dependencies: {}
# [doc = " Checks if the given expression is a call to `Default::default()`."] pub fn is_expr_default < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) -> bool { if let ExprKind :: Call (fn_expr , []) = & expr . kind && let ExprKind :: Path (qpath) = & fn_expr . kind && let Res :: Def (_ , def_id) = cx . qpath_res (qpath , fn_expr . hir_id) { cx . tcx . is_diagnostic_item (sym :: default_fn , def_id) } else { false } }
};
}
