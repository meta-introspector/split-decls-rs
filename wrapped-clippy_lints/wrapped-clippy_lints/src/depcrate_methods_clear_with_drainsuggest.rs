// Generated macro for suggest (function)
macro_rules! Depcrate_methods_clear_with_drainsuggest {
() => {
// Module: crate::methods::clear_with_drain
// Provides: {"suggest"}
// Dependencies: {}
fn suggest (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , span : Span) { if let Some (adt) = cx . typeck_results () . expr_ty (recv) . ty_adt_def () && let Some (ty_name) = cx . tcx . opt_item_name (adt . did ()) { span_lint_and_sugg (cx , CLEAR_WITH_DRAIN , span . with_hi (expr . span . hi ()) , format ! ("`drain` used to clear a `{ty_name}`") , "try" , "clear()" . to_string () , Applicability :: MachineApplicable ,) ; } }
};
}
