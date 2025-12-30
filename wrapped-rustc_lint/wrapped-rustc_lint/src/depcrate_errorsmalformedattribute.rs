// Generated macro for MalformedAttribute (struct)
macro_rules! Depcrate_errorsMalformedAttribute {
() => {
// Module: crate::errors
// Provides: {"MalformedAttribute"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (lint_malformed_attribute , code = E0452)] pub (crate) struct MalformedAttribute { # [primary_span] pub span : Span , # [subdiagnostic] pub sub : MalformedAttributeSub , }
};
}
