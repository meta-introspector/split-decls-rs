// Generated macro for InvalidTarget (struct)
macro_rules! Depcrate_session_diagnosticsInvalidTarget {
() => {
// Module: crate::session_diagnostics
// Provides: {"InvalidTarget"}
// Dependencies: {}
# [derive (Diagnostic)] # [help] # [diag (attr_parsing_invalid_target)] pub (crate) struct InvalidTarget { # [primary_span] # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub span : Span , pub name : AttrPath , pub target : & 'static str , pub applied : DiagArgValue , pub only : & 'static str , }
};
}
