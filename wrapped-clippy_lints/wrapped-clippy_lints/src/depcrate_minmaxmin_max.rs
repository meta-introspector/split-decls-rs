// Generated macro for min_max (function)
macro_rules! Depcrate_minmaxmin_max {
() => {
// Module: crate::minmax
// Provides: {"min_max"}
// Dependencies: {}
fn min_max < 'a > (cx : & LateContext < '_ > , expr : & 'a Expr < 'a >) -> Option < (MinMax , Constant , & 'a Expr < 'a >) > { match expr . kind { ExprKind :: Call (path , args) => { if let ExprKind :: Path (ref qpath) = path . kind { cx . typeck_results () . qpath_res (qpath , path . hir_id) . opt_def_id () . and_then (| def_id | match cx . tcx . get_diagnostic_name (def_id) { Some (sym :: cmp_min) => fetch_const (cx , expr . span . ctxt () , None , args , MinMax :: Min) , Some (sym :: cmp_max) => fetch_const (cx , expr . span . ctxt () , None , args , MinMax :: Max) , _ => None , }) } else { None } } , ExprKind :: MethodCall (path , receiver , args @ [_] , _) => { if cx . typeck_results () . expr_ty (receiver) . is_floating_point () || cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Ord) { match path . ident . name { sym :: max => fetch_const (cx , expr . span . ctxt () , Some (receiver) , args , MinMax :: Max) , sym :: min => fetch_const (cx , expr . span . ctxt () , Some (receiver) , args , MinMax :: Min) , _ => None , } } else { None } } , _ => None , } }
};
}
