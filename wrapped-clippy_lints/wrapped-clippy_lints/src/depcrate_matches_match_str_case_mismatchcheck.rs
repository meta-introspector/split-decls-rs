// Generated macro for check (function)
macro_rules! Depcrate_matches_match_str_case_mismatchcheck {
() => {
// Module: crate::matches::match_str_case_mismatch
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , scrutinee : & 'tcx Expr < '_ > , arms : & 'tcx [Arm < '_ >]) { if let ty :: Ref (_ , ty , _) = cx . typeck_results () . expr_ty (scrutinee) . kind () && let ty :: Str = ty . kind () { let mut visitor = MatchExprVisitor { cx } ; if let ControlFlow :: Break (case_method) = visitor . visit_expr (scrutinee) && let Some ((bad_case_span , bad_case_sym)) = verify_case (& case_method , arms) { lint (cx , & case_method , bad_case_span , bad_case_sym . as_str ()) ; } } }
};
}
