// Generated macro for EscapingBoundVarInTyOfAssocConstBinding (struct)
macro_rules! Depcrate_errorsEscapingBoundVarInTyOfAssocConstBinding {
() => {
// Module: crate::errors
// Provides: {"EscapingBoundVarInTyOfAssocConstBinding"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_escaping_bound_var_in_ty_of_assoc_const_binding)] pub (crate) struct EscapingBoundVarInTyOfAssocConstBinding < 'tcx > { # [primary_span] # [label] pub span : Span , pub assoc_const : Ident , pub var_name : Symbol , pub var_def_kind : & 'static str , # [label (hir_analysis_var_defined_here_label)] pub var_defined_here_label : Span , # [subdiagnostic] pub ty_note : Option < TyOfAssocConstBindingNote < 'tcx > > , }
};
}
