// Generated macro for impl_3399 (impl)
macro_rules! Depcrate_let_with_type_underscoreimpl_3399 {
() => {
// Module: crate::let_with_type_underscore
// Provides: {"impl_3399"}
// Dependencies: {}
impl EarlyLintPass for UnderscoreTyped { fn check_local (& mut self , cx : & EarlyContext < '_ > , local : & Local) { if let Some (ty) = & local . ty && let TyKind :: Infer = ty . kind && local . span . eq_ctxt (ty . span) && let sm = cx . sess () . source_map () && ! local . span . in_external_macro (sm) && ! is_from_proc_macro (cx , & * * ty) { let span_to_remove = sm . span_extend_to_prev_char_before (ty . span , ':' , true) . with_leading_whitespace (cx) . into_span () ; span_lint_and_then (cx , LET_WITH_TYPE_UNDERSCORE , local . span , "variable declared with type underscore" , | diag | { diag . span_suggestion_verbose (span_to_remove , "remove the explicit type `_` declaration" , "" , Applicability :: MachineApplicable ,) ; } ,) ; } } }
};
}
