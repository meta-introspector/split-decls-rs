// Generated macro for AmbiguousNegativeLiteralsCurrentBehaviorSuggestion (struct)
macro_rules! Depcrate_lintsAmbiguousNegativeLiteralsCurrentBehaviorSuggestion {
() => {
// Module: crate::lints
// Provides: {"AmbiguousNegativeLiteralsCurrentBehaviorSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (lint_current_behavior , applicability = "maybe-incorrect")] pub (crate) struct AmbiguousNegativeLiteralsCurrentBehaviorSuggestion { # [suggestion_part (code = "(")] pub start_span : Span , # [suggestion_part (code = ")")] pub end_span : Span , }
};
}
