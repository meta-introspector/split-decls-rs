// Generated macro for UseLetUnderscoreIgnoreSuggestion (enum)
macro_rules! Depcrate_lintsUseLetUnderscoreIgnoreSuggestion {
() => {
// Module: crate::lints
// Provides: {"UseLetUnderscoreIgnoreSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum UseLetUnderscoreIgnoreSuggestion { # [note (lint_use_let_underscore_ignore_suggestion)] Note , # [multipart_suggestion (lint_use_let_underscore_ignore_suggestion , style = "verbose" , applicability = "maybe-incorrect")] Suggestion { # [suggestion_part (code = "let _ = ")] start_span : Span , # [suggestion_part (code = "")] end_span : Span , } , }
};
}
