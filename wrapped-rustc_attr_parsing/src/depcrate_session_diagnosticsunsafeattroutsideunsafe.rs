// Generated macro for UnsafeAttrOutsideUnsafe (struct)
macro_rules! Depcrate_session_diagnosticsUnsafeAttrOutsideUnsafe {
() => {
// Module: crate::session_diagnostics
// Provides: {"UnsafeAttrOutsideUnsafe"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_unsafe_attr_outside_unsafe)] pub (crate) struct UnsafeAttrOutsideUnsafe { # [primary_span] # [label] pub span : Span , # [subdiagnostic] pub suggestion : UnsafeAttrOutsideUnsafeSuggestion , }
};
}
