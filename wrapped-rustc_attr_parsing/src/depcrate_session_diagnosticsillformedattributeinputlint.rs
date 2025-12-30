// Generated macro for IllFormedAttributeInputLint (struct)
macro_rules! Depcrate_session_diagnosticsIllFormedAttributeInputLint {
() => {
// Module: crate::session_diagnostics
// Provides: {"IllFormedAttributeInputLint"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_ill_formed_attribute_input)] pub (crate) struct IllFormedAttributeInputLint { # [primary_span] pub span : Span , pub num_suggestions : usize , pub suggestions : DiagArgValue , }
};
}
