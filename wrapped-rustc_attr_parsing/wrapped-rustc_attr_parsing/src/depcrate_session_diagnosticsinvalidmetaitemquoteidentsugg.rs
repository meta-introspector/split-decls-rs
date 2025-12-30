// Generated macro for InvalidMetaItemQuoteIdentSugg (struct)
macro_rules! Depcrate_session_diagnosticsInvalidMetaItemQuoteIdentSugg {
() => {
// Module: crate::session_diagnostics
// Provides: {"InvalidMetaItemQuoteIdentSugg"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (attr_parsing_quote_ident_sugg , applicability = "machine-applicable")] pub (crate) struct InvalidMetaItemQuoteIdentSugg { # [suggestion_part (code = "\"")] pub before : Span , # [suggestion_part (code = "\"")] pub after : Span , }
};
}
