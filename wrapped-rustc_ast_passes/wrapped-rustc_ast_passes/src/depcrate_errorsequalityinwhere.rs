// Generated macro for EqualityInWhere (struct)
macro_rules! Depcrate_errorsEqualityInWhere {
() => {
// Module: crate::errors
// Provides: {"EqualityInWhere"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_equality_in_where)] # [note] pub (crate) struct EqualityInWhere { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub assoc : Option < AssociatedSuggestion > , # [subdiagnostic] pub assoc2 : Option < AssociatedSuggestion2 > , }
};
}
