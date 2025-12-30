// Generated macro for StackPopInfo (struct)
macro_rules! Depcrate_interpret_stackStackPopInfo {
() => {
// Module: crate::interpret::stack
// Provides: {"StackPopInfo"}
// Dependencies: {}
# [doc = " Return type of [`InterpCx::pop_stack_frame_raw`]."] pub struct StackPopInfo < 'tcx , Prov : Provenance > { # [doc = " Additional information about the action to be performed when returning from the popped"] # [doc = " stack frame."] pub return_action : ReturnAction , # [doc = " [`return_cont`](Frame::return_cont) of the popped stack frame."] pub return_cont : ReturnContinuation , # [doc = " [`return_place`](Frame::return_place) of the popped stack frame."] pub return_place : PlaceTy < 'tcx , Prov > , }
};
}
