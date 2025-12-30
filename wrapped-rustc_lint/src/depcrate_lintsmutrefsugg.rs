// Generated macro for MutRefSugg (enum)
macro_rules! Depcrate_lintsMutRefSugg {
() => {
// Module: crate::lints
// Provides: {"MutRefSugg"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum MutRefSugg { # [multipart_suggestion (lint_suggestion , style = "verbose" , applicability = "maybe-incorrect")] Shared { # [suggestion_part (code = "&raw const ")] span : Span , } , # [multipart_suggestion (lint_suggestion_mut , style = "verbose" , applicability = "maybe-incorrect")] Mut { # [suggestion_part (code = "&raw mut ")] span : Span , } , }
};
}
