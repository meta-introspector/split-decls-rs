// Generated macro for check (function)
macro_rules! Depcrate_methods_option_as_ref_clonedcheck {
() => {
// Module: crate::methods::option_as_ref_cloned
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , cloned_ident_span : Span , as_ref_method : Symbol , as_ref_recv : & Expr < '_ > , as_ref_ident_span : Span ,) { if cx . typeck_results () . expr_ty (as_ref_recv) . peel_refs () . is_diag_item (cx , sym :: Option) { span_lint_and_sugg (cx , OPTION_AS_REF_CLONED , as_ref_ident_span . to (cloned_ident_span) , format ! ("cloning an `Option<_>` using `.{as_ref_method}().cloned()`") , "this can be written more concisely by cloning the `Option<_>` directly" , "clone" . into () , Applicability :: MachineApplicable ,) ; } }
};
}
