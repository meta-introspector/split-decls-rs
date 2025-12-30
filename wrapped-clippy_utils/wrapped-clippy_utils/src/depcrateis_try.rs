// Generated macro for is_try (function)
macro_rules! Depcrateis_try {
() => {
// Module: crate
// Provides: {"is_try"}
// Dependencies: {}
# [doc = " Checks if a given expression is a match expression expanded from the `?`"] # [doc = " operator or the `try` macro."] pub fn is_try < 'tcx > (cx : & LateContext < '_ > , expr : & 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { fn is_ok (cx : & LateContext < '_ > , arm : & Arm < '_ >) -> bool { if let PatKind :: TupleStruct (ref path , pat , ddpos) = arm . pat . kind && ddpos . as_opt_usize () . is_none () && cx . qpath_res (path , arm . pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , ResultOk) && let PatKind :: Binding (_ , hir_id , _ , None) = pat [0] . kind && arm . body . res_local_id () == Some (hir_id) { return true ; } false } fn is_err (cx : & LateContext < '_ > , arm : & Arm < '_ >) -> bool { if let PatKind :: TupleStruct (ref path , _ , _) = arm . pat . kind { cx . qpath_res (path , arm . pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , ResultErr) } else { false } } if let ExprKind :: Match (_ , arms , ref source) = expr . kind { if let MatchSource :: TryDesugar (_) = * source { return Some (expr) ; } if arms . len () == 2 && arms [0] . guard . is_none () && arms [1] . guard . is_none () && ((is_ok (cx , & arms [0]) && is_err (cx , & arms [1])) || (is_ok (cx , & arms [1]) && is_err (cx , & arms [0]))) { return Some (expr) ; } } None }
};
}
