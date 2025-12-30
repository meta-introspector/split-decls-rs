// Generated macro for pat_allowed_for_else (function)
macro_rules! Depcrate_manual_let_elsepat_allowed_for_else {
() => {
// Module: crate::manual_let_else
// Provides: {"pat_allowed_for_else"}
// Dependencies: {}
fn pat_allowed_for_else (cx : & LateContext < '_ > , pat : & '_ Pat < '_ > , check_types : bool) -> bool { let mut has_bindings = false ; pat . each_binding_or_first (& mut | _ , _ , _ , _ | has_bindings = true) ; if has_bindings { return false ; } if ! check_types { return true ; } let typeck_results = cx . typeck_results () ; let mut has_disallowed = false ; pat . walk_always (| pat | { if ! matches ! (pat . kind , PatKind :: Struct (..) | PatKind :: TupleStruct (..) | PatKind :: Expr (PatExpr { kind : PatExprKind :: Path (..) , .. } ,)) { return ; } let ty = typeck_results . pat_ty (pat) ; if ! (ty . is_diag_item (cx , sym :: Option) || ty . is_diag_item (cx , sym :: Result)) { has_disallowed = true ; } }) ; ! has_disallowed }
};
}
