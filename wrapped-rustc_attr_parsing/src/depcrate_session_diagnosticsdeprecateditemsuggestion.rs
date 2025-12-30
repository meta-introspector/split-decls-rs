// Generated macro for DeprecatedItemSuggestion (struct)
macro_rules! Depcrate_session_diagnosticsDeprecatedItemSuggestion {
() => {
// Module: crate::session_diagnostics
// Provides: {"DeprecatedItemSuggestion"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_deprecated_item_suggestion)] pub (crate) struct DeprecatedItemSuggestion { # [primary_span] pub span : Span , # [help] pub is_nightly : bool , # [note] pub details : () , }
};
}
