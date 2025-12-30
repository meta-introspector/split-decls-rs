// Generated macro for PlaceholderNotAllowedItemSignatures (struct)
macro_rules! Depcrate_errorsPlaceholderNotAllowedItemSignatures {
() => {
// Module: crate::errors
// Provides: {"PlaceholderNotAllowedItemSignatures"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_placeholder_not_allowed_item_signatures , code = E0121)] pub (crate) struct PlaceholderNotAllowedItemSignatures { # [primary_span] # [label] pub spans : Vec < Span > , pub kind : String , }
};
}
