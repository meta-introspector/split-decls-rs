// Generated macro for FlowControl (struct)
macro_rules! Depcrate_proto_streams_flow_controlFlowControl {
() => {
// Module: crate::proto::streams::flow_control
// Provides: {"FlowControl"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub struct FlowControl { # [doc = " Window the peer knows about."] # [doc = ""] # [doc = " This can go negative if a SETTINGS_INITIAL_WINDOW_SIZE is received."] # [doc = ""] # [doc = " For example, say the peer sends a request and uses 32kb of the window."] # [doc = " We send a SETTINGS_INITIAL_WINDOW_SIZE of 16kb. The peer has to adjust"] # [doc = " its understanding of the capacity of the window, and that would be:"] # [doc = ""] # [doc = " ```notrust"] # [doc = " default (64kb) - used (32kb) - settings_diff (64kb - 16kb): -16kb"] # [doc = " ```"] window_size : Window , # [doc = " Window that we know about."] # [doc = ""] # [doc = " This can go negative if a user declares a smaller target window than"] # [doc = " the peer knows about."] available : Window , }
};
}
