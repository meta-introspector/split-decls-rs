// Generated macro for check_match (function)
macro_rules! Depcrate_matches_manual_ok_errcheck_match {
() => {
// Module: crate::matches::manual_ok_err
// Provides: {"check_match"}
// Dependencies: {}
pub (crate) fn check_match (cx : & LateContext < '_ > , expr : & Expr < '_ > , scrutinee : & Expr < '_ > , arms : & [Arm < '_ >]) { if let Some (inner_expr_ty) = option_arg_ty (cx , cx . typeck_results () . expr_ty (expr)) && arms . len () == 2 && arms . iter () . all (| arm | arm . guard . is_none ()) && let Some ((idx , is_ok)) = arms . iter () . enumerate () . find_map (| (arm_idx , arm) | { if let Some ((is_ok , ident)) = is_ok_or_err (cx , arm . pat) && is_some_ident (cx , arm . body , ident , inner_expr_ty) { Some ((arm_idx , is_ok)) } else { None } }) && is_variant_or_wildcard (cx , arms [1 - idx] . pat , idx == 0 , is_ok) && is_none (cx , arms [1 - idx] . body) { apply_lint (cx , expr , scrutinee , is_ok) ; } }
};
}
