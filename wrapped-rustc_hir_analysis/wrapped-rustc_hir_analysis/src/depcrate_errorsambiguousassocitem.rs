// Generated macro for AmbiguousAssocItem (struct)
macro_rules! Depcrate_errorsAmbiguousAssocItem {
() => {
// Module: crate::errors
// Provides: {"AmbiguousAssocItem"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_ambiguous_assoc_item)] pub (crate) struct AmbiguousAssocItem < 'a > { # [primary_span] # [label] pub span : Span , pub assoc_kind : & 'static str , pub assoc_ident : Ident , pub qself : & 'a str , }
};
}
