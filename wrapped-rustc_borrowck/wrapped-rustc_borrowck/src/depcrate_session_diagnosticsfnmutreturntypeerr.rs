// Generated macro for FnMutReturnTypeErr (enum)
macro_rules! Depcrate_session_diagnosticsFnMutReturnTypeErr {
() => {
// Module: crate::session_diagnostics
// Provides: {"FnMutReturnTypeErr"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum FnMutReturnTypeErr { # [label (borrowck_returned_closure_escaped)] ReturnClosure { # [primary_span] span : Span , } , # [label (borrowck_returned_async_block_escaped)] ReturnAsyncBlock { # [primary_span] span : Span , } , # [label (borrowck_returned_ref_escaped)] ReturnRef { # [primary_span] span : Span , } , }
};
}
