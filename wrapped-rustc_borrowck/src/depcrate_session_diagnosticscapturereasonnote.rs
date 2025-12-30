// Generated macro for CaptureReasonNote (enum)
macro_rules! Depcrate_session_diagnosticsCaptureReasonNote {
() => {
// Module: crate::session_diagnostics
// Provides: {"CaptureReasonNote"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum CaptureReasonNote { # [note (borrowck_moved_a_fn_once_in_call)] FnOnceMoveInCall { # [primary_span] var_span : Span , } , # [note (borrowck_calling_operator_moves)] UnOpMoveByOperator { # [primary_span] span : Span , } , # [note (borrowck_calling_operator_moves_lhs)] LhsMoveByOperator { # [primary_span] span : Span , } , # [note (borrowck_func_take_self_moved_place)] FuncTakeSelf { func : String , place_name : String , # [primary_span] span : Span , } , }
};
}
