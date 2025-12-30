// Generated macro for handle_action_loop_error (function)
macro_rules! Depcrate_prompts_h3handle_action_loop_error {
() => {
// Module: crate::prompts::h3
// Provides: {"handle_action_loop_error"}
// Dependencies: {}
fn handle_action_loop_error (err : InquireError) -> bool { match err { inquire :: InquireError :: OperationCanceled | inquire :: InquireError :: OperationInterrupted => false , _ => { println ! ("Unexpected error: {err}") ; true } , } }
};
}
