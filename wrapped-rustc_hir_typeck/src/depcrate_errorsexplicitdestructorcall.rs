// Generated macro for ExplicitDestructorCall (struct)
macro_rules! Depcrate_errorsExplicitDestructorCall {
() => {
// Module: crate::errors
// Provides: {"ExplicitDestructorCall"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_explicit_destructor , code = E0040)] pub (crate) struct ExplicitDestructorCall { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub sugg : ExplicitDestructorCallSugg , }
};
}
