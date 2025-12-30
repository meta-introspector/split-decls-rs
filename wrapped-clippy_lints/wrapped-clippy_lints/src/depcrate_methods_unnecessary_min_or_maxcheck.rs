// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_min_or_maxcheck {
() => {
// Module: crate::methods::unnecessary_min_or_max
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , name : Symbol , recv : & 'tcx Expr < '_ > , arg : & 'tcx Expr < '_ > ,) { let typeck_results = cx . typeck_results () ; let ecx = ConstEvalCtxt :: with_env (cx . tcx , cx . typing_env () , typeck_results) ; if let Some (id) = typeck_results . type_dependent_def_id (expr . hir_id) && let Some (fn_name) = cx . tcx . get_diagnostic_name (id) && matches ! (fn_name , sym :: cmp_ord_min | sym :: cmp_ord_max) { let ctxt = expr . span . ctxt () ; if let Some (left) = ecx . eval_local (recv , ctxt) && let Some (right) = ecx . eval_local (arg , ctxt) { let Some (ord) = Constant :: partial_cmp (cx . tcx , typeck_results . expr_ty (recv) , & left , & right) else { return ; } ; lint (cx , expr , name , recv . span , arg . span , ord) ; } else if let Some (extrema) = detect_extrema (cx , recv) { let ord = match extrema { Extrema :: Minimum => Ordering :: Less , Extrema :: Maximum => Ordering :: Greater , } ; lint (cx , expr , name , recv . span , arg . span , ord) ; } else if let Some (extrema) = detect_extrema (cx , arg) { let ord = match extrema { Extrema :: Minimum => Ordering :: Greater , Extrema :: Maximum => Ordering :: Less , } ; lint (cx , expr , name , recv . span , arg . span , ord) ; } } }
};
}
