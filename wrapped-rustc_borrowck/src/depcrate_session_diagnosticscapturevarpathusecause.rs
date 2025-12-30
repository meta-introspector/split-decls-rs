// Generated macro for CaptureVarPathUseCause (enum)
macro_rules! Depcrate_session_diagnosticsCaptureVarPathUseCause {
() => {
// Module: crate::session_diagnostics
// Provides: {"CaptureVarPathUseCause"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum CaptureVarPathUseCause { # [label (borrowck_borrow_due_to_use_coroutine)] BorrowInCoroutine { # [primary_span] path_span : Span , } , # [label (borrowck_use_due_to_use_coroutine)] UseInCoroutine { # [primary_span] path_span : Span , } , # [label (borrowck_assign_due_to_use_coroutine)] AssignInCoroutine { # [primary_span] path_span : Span , } , # [label (borrowck_assign_part_due_to_use_coroutine)] AssignPartInCoroutine { # [primary_span] path_span : Span , } , # [label (borrowck_borrow_due_to_use_closure)] BorrowInClosure { # [primary_span] path_span : Span , } , # [label (borrowck_use_due_to_use_closure)] UseInClosure { # [primary_span] path_span : Span , } , # [label (borrowck_assign_due_to_use_closure)] AssignInClosure { # [primary_span] path_span : Span , } , # [label (borrowck_assign_part_due_to_use_closure)] AssignPartInClosure { # [primary_span] path_span : Span , } , }
};
}
