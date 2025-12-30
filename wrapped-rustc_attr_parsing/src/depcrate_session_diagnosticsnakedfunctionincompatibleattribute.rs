// Generated macro for NakedFunctionIncompatibleAttribute (struct)
macro_rules! Depcrate_session_diagnosticsNakedFunctionIncompatibleAttribute {
() => {
// Module: crate::session_diagnostics
// Provides: {"NakedFunctionIncompatibleAttribute"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_naked_functions_incompatible_attribute , code = E0736)] pub (crate) struct NakedFunctionIncompatibleAttribute { # [primary_span] # [label] pub span : Span , # [label (attr_parsing_naked_attribute)] pub naked_span : Span , pub attr : String , }
};
}
