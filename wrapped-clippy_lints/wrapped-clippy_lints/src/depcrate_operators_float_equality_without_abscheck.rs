// Generated macro for check (function)
macro_rules! Depcrate_operators_float_equality_without_abscheck {
() => {
// Module: crate::operators::float_equality_without_abs
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , op : BinOpKind , lhs : & 'tcx Expr < '_ > , rhs : & 'tcx Expr < '_ > ,) { let (lhs , rhs) = match op { BinOpKind :: Lt => (lhs , rhs) , BinOpKind :: Gt => (rhs , lhs) , _ => return , } ; if let ExprKind :: Binary (Spanned { node : BinOpKind :: Sub , .. } , val_l , val_r ,) = lhs . kind && let ExprKind :: Path (ref epsilon_path) = rhs . kind && let Res :: Def (DefKind :: AssocConst , def_id) = cx . qpath_res (epsilon_path , rhs . hir_id) && let Some (sym) = cx . tcx . get_diagnostic_name (def_id) && matches ! (sym , sym :: f16_epsilon | sym :: f32_epsilon | sym :: f64_epsilon | sym :: f128_epsilon) && let t_val_l = cx . typeck_results () . expr_ty (val_l) && let t_val_r = cx . typeck_results () . expr_ty (val_r) && let ty :: Float (_) = t_val_l . kind () && let ty :: Float (_) = t_val_r . kind () { let sug_l = sugg :: Sugg :: hir (cx , val_l , "..") ; let sug_r = sugg :: Sugg :: hir (cx , val_r , "..") ; let suggestion = format ! ("{}.abs()" , sugg :: make_assoc (AssocOp :: Binary (BinOpKind :: Sub) , & sug_l , & sug_r) . maybe_paren ()) ; span_lint_and_then (cx , FLOAT_EQUALITY_WITHOUT_ABS , expr . span , "float equality check without `.abs()`" , | diag | { diag . span_suggestion (lhs . span , "add `.abs()`" , suggestion , Applicability :: MaybeIncorrect) ; } ,) ; } }
};
}
