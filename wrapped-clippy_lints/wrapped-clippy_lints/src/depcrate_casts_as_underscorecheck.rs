// Generated macro for check (function)
macro_rules! Depcrate_casts_as_underscorecheck {
() => {
// Module: crate::casts::as_underscore
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , ty : & 'tcx Ty < '_ >) { if matches ! (ty . kind , TyKind :: Infer (())) { span_lint_and_then (cx , AS_UNDERSCORE , expr . span , "using `as _` conversion" , | diag | { let ty_resolved = cx . typeck_results () . expr_ty (expr) ; if ty_resolved . is_suggestable (cx . tcx , true) { diag . span_suggestion (ty . span , "consider giving the type explicitly" , ty_resolved , Applicability :: MachineApplicable ,) ; } else { diag . help ("consider giving the type explicitly") ; } }) ; } }
};
}
