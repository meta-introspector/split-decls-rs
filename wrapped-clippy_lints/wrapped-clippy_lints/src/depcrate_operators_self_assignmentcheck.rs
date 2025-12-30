// Generated macro for check (function)
macro_rules! Depcrate_operators_self_assignmentcheck {
() => {
// Module: crate::operators::self_assignment
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , lhs : & 'tcx Expr < '_ > , rhs : & 'tcx Expr < '_ >) { if eq_expr_value (cx , lhs , rhs) { let lhs = snippet (cx , lhs . span , "<lhs>") ; let rhs = snippet (cx , rhs . span , "<rhs>") ; span_lint (cx , SELF_ASSIGNMENT , e . span , format ! ("self-assignment of `{rhs}` to `{lhs}`") ,) ; } }
};
}
