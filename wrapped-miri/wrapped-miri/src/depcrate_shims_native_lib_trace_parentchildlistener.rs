// Generated macro for ChildListener (struct)
macro_rules! Depcrate_shims_native_lib_trace_parentChildListener {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"ChildListener"}
// Dependencies: {}
# [doc = " A listener for the FFI start info channel along with relevant state."] pub struct ChildListener { # [doc = " The matching channel for the child's `Supervisor` struct."] message_rx : ipc :: IpcReceiver < TraceRequest > , # [doc = " ..."] confirm_tx : ipc :: IpcSender < Confirmation > , # [doc = " Whether an FFI call is currently ongoing."] attached : bool , # [doc = " If `Some`, overrides the return code with the given value."] override_retcode : Option < i32 > , # [doc = " Last code obtained from a child exiting."] last_code : Option < i32 > , }
};
}
