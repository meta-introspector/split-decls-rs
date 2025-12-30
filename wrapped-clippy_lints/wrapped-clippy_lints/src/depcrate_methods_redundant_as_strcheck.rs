// Generated macro for check (function)
macro_rules! Depcrate_methods_redundant_as_strcheck {
() => {
// Module: crate::methods::redundant_as_str
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , _expr : & Expr < '_ > , recv : & Expr < '_ > , as_str_span : Span , other_method_span : Span ,) { if cx . typeck_results () . expr_ty (recv) . ty_adt_def () . is_some_and (| adt | Some (adt . did ()) == cx . tcx . lang_items () . string ()) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , REDUNDANT_AS_STR , as_str_span . to (other_method_span) , "this `as_str` is redundant and can be removed as the method immediately following exists on `String` too" , "try" , snippet_with_applicability (cx , other_method_span , ".." , & mut applicability) . into_owned () , applicability ,) ; } }
};
}
