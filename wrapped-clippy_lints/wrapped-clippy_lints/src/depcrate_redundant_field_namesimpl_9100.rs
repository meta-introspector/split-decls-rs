// Generated macro for impl_9100 (impl)
macro_rules! Depcrate_redundant_field_namesimpl_9100 {
() => {
// Module: crate::redundant_field_names
// Provides: {"impl_9100"}
// Dependencies: {}
impl EarlyLintPass for RedundantFieldNames { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if ! self . msrv . meets (msrvs :: FIELD_INIT_SHORTHAND) { return ; } if expr . span . in_external_macro (cx . sess () . source_map ()) { return ; } if let ExprKind :: Struct (ref se) = expr . kind { for field in & se . fields { if ! field . is_shorthand && let ExprKind :: Path (None , path) = & field . expr . kind && let [segment] = path . segments . as_slice () && segment . args . is_none () && segment . ident == field . ident && field . span . eq_ctxt (field . ident . span) { span_lint_and_sugg (cx , REDUNDANT_FIELD_NAMES , field . span , "redundant field names in struct initialization" , "replace it with" , field . ident . to_string () , Applicability :: MachineApplicable ,) ; } } } } extract_msrv_attr ! () ; }
};
}
