// Generated macro for MissingOneOfTraitItem (struct)
macro_rules! Depcrate_errorsMissingOneOfTraitItem {
() => {
// Module: crate::errors
// Provides: {"MissingOneOfTraitItem"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_missing_one_of_trait_item , code = E0046)] pub (crate) struct MissingOneOfTraitItem { # [primary_span] # [label] pub span : Span , # [note] pub note : Option < Span > , pub missing_items_msg : String , }
};
}
