// Generated macro for UnknownCrateTypesSub (struct)
macro_rules! Depcrate_lintsUnknownCrateTypesSub {
() => {
// Module: crate::lints
// Provides: {"UnknownCrateTypesSub"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [suggestion (lint_suggestion , code = r#""{candidate}""# , applicability = "maybe-incorrect")] pub (crate) struct UnknownCrateTypesSub { # [primary_span] pub span : Span , pub candidate : Symbol , }
};
}
