// Generated macro for AssocItemConstraintsNotAllowedHere (struct)
macro_rules! Depcrate_errorsAssocItemConstraintsNotAllowedHere {
() => {
// Module: crate::errors
// Provides: {"AssocItemConstraintsNotAllowedHere"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_assoc_item_constraints_not_allowed_here , code = E0229)] pub (crate) struct AssocItemConstraintsNotAllowedHere { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub fn_trait_expansion : Option < ParenthesizedFnTraitExpansion > , }
};
}
