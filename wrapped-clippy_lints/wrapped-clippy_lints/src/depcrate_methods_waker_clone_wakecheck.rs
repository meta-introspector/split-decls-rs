// Generated macro for check (function)
macro_rules! Depcrate_methods_waker_clone_wakecheck {
() => {
// Module: crate::methods::waker_clone_wake
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , recv : & 'tcx Expr < '_ >) { let ty = cx . typeck_results () . expr_ty (recv) ; if let Some (did) = ty . ty_adt_def () && cx . tcx . is_diagnostic_item (sym :: Waker , did . did ()) && let ExprKind :: MethodCall (_ , waker_ref , & [] , _) = recv . kind && cx . ty_based_def (recv) . opt_parent (cx) . is_diag_item (cx , sym :: Clone) { let mut applicability = Applicability :: MachineApplicable ; let snippet = snippet_with_applicability (cx , waker_ref . span . source_callsite () , ".." , & mut applicability) ; span_lint_and_sugg (cx , WAKER_CLONE_WAKE , expr . span , "cloning a `Waker` only to wake it" , "replace with" , format ! ("{snippet}.wake_by_ref()") , applicability ,) ; } }
};
}
