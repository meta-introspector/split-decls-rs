// Generated macro for OnClosureNote (enum)
macro_rules! Depcrate_session_diagnosticsOnClosureNote {
() => {
// Module: crate::session_diagnostics
// Provides: {"OnClosureNote"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum OnClosureNote < 'a > { # [note (borrowck_closure_invoked_twice)] InvokedTwice { place_name : & 'a str , # [primary_span] span : Span , } , # [note (borrowck_closure_moved_twice)] MovedTwice { place_name : & 'a str , # [primary_span] span : Span , } , }
};
}
