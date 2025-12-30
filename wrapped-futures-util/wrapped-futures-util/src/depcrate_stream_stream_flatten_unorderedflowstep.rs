// Generated macro for FlowStep (enum)
macro_rules! Depcrate_stream_stream_flatten_unorderedFlowStep {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"FlowStep"}
// Dependencies: {}
# [doc = " Describes the next flow step."] # [derive (Debug , Clone)] pub enum FlowStep < C , R > { # [doc = " Just yields an item and continues standard flow."] Continue (C) , # [doc = " Immediately returns an underlying item from the function."] Return (R) , }
};
}
