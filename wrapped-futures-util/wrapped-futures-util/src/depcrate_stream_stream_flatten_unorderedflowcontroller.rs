// Generated macro for FlowController (trait)
macro_rules! Depcrate_stream_stream_flatten_unorderedFlowController {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"FlowController"}
// Dependencies: {}
# [doc = " Returns the next flow step based on the received item."] pub trait FlowController < I , O > { # [doc = " Handles an item producing `FlowStep` describing the next flow step."] fn next_step (item : I) -> FlowStep < I , O > ; }
};
}
