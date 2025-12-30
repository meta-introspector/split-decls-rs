// Generated macro for AmbiguousNegativeLiteralsNegativeLiteralSuggestion (struct)
macro_rules! Depcrate_lintsAmbiguousNegativeLiteralsNegativeLiteralSuggestion {
() => {
// Module: crate::lints
// Provides: {"AmbiguousNegativeLiteralsNegativeLiteralSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (lint_negative_literal , applicability = "maybe-incorrect")] pub (crate) struct AmbiguousNegativeLiteralsNegativeLiteralSuggestion { # [suggestion_part (code = "(")] pub start_span : Span , # [suggestion_part (code = ")")] pub end_span : Span , }
};
}
