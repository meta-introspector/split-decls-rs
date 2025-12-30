// Generated macro for FormatUnknownTraitSugg (struct)
macro_rules! Depcrate_errorsFormatUnknownTraitSugg {
() => {
// Module: crate::errors
// Provides: {"FormatUnknownTraitSugg"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [suggestion (builtin_macros_suggestion , code = "{fmt}" , style = "tool-only" , applicability = "maybe-incorrect")] pub (crate) struct FormatUnknownTraitSugg { # [primary_span] pub span : Span , pub fmt : & 'static str , pub trait_name : & 'static str , }
};
}
