// Generated macro for GenericDoesNotLiveLongEnough (struct)
macro_rules! Depcrate_session_diagnosticsGenericDoesNotLiveLongEnough {
() => {
// Module: crate::session_diagnostics
// Provides: {"GenericDoesNotLiveLongEnough"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (borrowck_generic_does_not_live_long_enough)] pub (crate) struct GenericDoesNotLiveLongEnough { pub kind : String , # [primary_span] pub span : Span , }
};
}
