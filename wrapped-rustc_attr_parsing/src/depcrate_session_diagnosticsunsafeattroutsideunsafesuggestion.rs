// Generated macro for UnsafeAttrOutsideUnsafeSuggestion (struct)
macro_rules! Depcrate_session_diagnosticsUnsafeAttrOutsideUnsafeSuggestion {
() => {
// Module: crate::session_diagnostics
// Provides: {"UnsafeAttrOutsideUnsafeSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (attr_parsing_unsafe_attr_outside_unsafe_suggestion , applicability = "machine-applicable")] pub (crate) struct UnsafeAttrOutsideUnsafeSuggestion { # [suggestion_part (code = "unsafe(")] pub left : Span , # [suggestion_part (code = ")")] pub right : Span , }
};
}
