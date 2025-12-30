// Generated macro for CaptureReasonLabel (enum)
macro_rules! Depcrate_session_diagnosticsCaptureReasonLabel {
() => {
// Module: crate::session_diagnostics
// Provides: {"CaptureReasonLabel"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum CaptureReasonLabel < 'a > { # [label (borrowck_moved_due_to_call)] Call { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_moved_due_to_usage_in_operator)] OperatorUse { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_moved_due_to_implicit_into_iter_call)] ImplicitCall { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_moved_due_to_method_call)] MethodCall { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_moved_due_to_await)] Await { # [primary_span] fn_call_span : Span , place_name : & 'a str , is_partial : bool , is_loop_message : bool , } , # [label (borrowck_value_moved_here)] MovedHere { # [primary_span] move_span : Span , is_partial : bool , is_move_msg : bool , is_loop_message : bool , } , # [label (borrowck_consider_borrow_type_contents)] BorrowContent { # [primary_span] var_span : Span , } , }
};
}
