// Generated macro for check_method_calls (function)
macro_rules! Depcrate_matches_redundant_guardscheck_method_calls {
() => {
// Module: crate::matches::redundant_guards
// Provides: {"check_method_calls"}
// Dependencies: {}
fn check_method_calls < 'tcx > (cx : & LateContext < 'tcx > , arm : & Arm < 'tcx > , method : Symbol , recv : & Expr < '_ > , args : & [Expr < '_ >] , if_expr : & Expr < '_ > , binding : & PatBindingInfo ,) { let ty = cx . typeck_results () . expr_ty (recv) . peel_refs () ; let slice_like = ty . is_slice () || ty . is_array () ; let sugg = if method == sym :: is_empty { if ty . is_str () && ! is_in_const_context (cx) { r#""""# . into () } else if slice_like { "[]" . into () } else { return ; } } else if slice_like && let Some (needle) = args . first () && let ExprKind :: AddrOf (.. , needle) = needle . kind && let ExprKind :: Array (needles) = needle . kind && needles . iter () . all (| needle | expr_can_be_pat (cx , needle)) { let mut sugg = snippet (cx , needle . span , "<needle>") . into_owned () ; if needles . is_empty () { sugg . insert_str (1 , "..") ; } else if method == sym :: starts_with { sugg . insert_str (sugg . len () - 1 , ", ..") ; } else if method == sym :: ends_with { sugg . insert_str (1 , ".., ") ; } else { return ; } sugg . into () } else { return ; } ; emit_redundant_guards (cx , arm , if_expr . span , sugg , binding , None) ; }
};
}
