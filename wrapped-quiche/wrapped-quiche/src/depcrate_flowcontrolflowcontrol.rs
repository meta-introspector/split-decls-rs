// Generated macro for FlowControl (struct)
macro_rules! Depcrate_flowcontrolFlowControl {
() => {
// Module: crate::flowcontrol
// Provides: {"FlowControl"}
// Dependencies: {}
# [derive (Default , Debug)] pub struct FlowControl { # [doc = " Total consumed bytes by the receiver."] consumed : u64 , # [doc = " Flow control limit."] max_data : u64 , # [doc = " The receive window. This value is used for updating"] # [doc = " flow control limit."] window : u64 , # [doc = " The maximum receive window."] max_window : u64 , # [doc = " Last update time of max_data for autotuning the window."] last_update : Option < Instant > , }
};
}
