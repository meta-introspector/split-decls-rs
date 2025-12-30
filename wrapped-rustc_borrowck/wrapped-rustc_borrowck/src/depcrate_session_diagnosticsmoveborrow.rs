// Generated macro for MoveBorrow (struct)
macro_rules! Depcrate_session_diagnosticsMoveBorrow {
() => {
// Module: crate::session_diagnostics
// Provides: {"MoveBorrow"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (borrowck_cannot_move_when_borrowed , code = E0505)] pub (crate) struct MoveBorrow < 'a > { pub place : & 'a str , pub borrow_place : & 'a str , pub value_place : & 'a str , # [primary_span] # [label (borrowck_move_label)] pub span : Span , # [label] pub borrow_span : Span , }
};
}
