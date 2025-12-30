// Generated macro for CastUnknownPointer (struct)
macro_rules! Depcrate_errorsCastUnknownPointer {
() => {
// Module: crate::errors
// Provides: {"CastUnknownPointer"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_cast_unknown_pointer , code = E0641)] pub (crate) struct CastUnknownPointer { # [primary_span] pub span : Span , pub to : bool , # [subdiagnostic] pub sub : CastUnknownPointerSub , }
};
}
