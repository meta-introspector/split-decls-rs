// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_shims_native_lib_trace_parentimpl_1026 {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"impl_1026"}
// Dependencies: {}
impl ChildListener { pub fn new (message_rx : ipc :: IpcReceiver < TraceRequest > , confirm_tx : ipc :: IpcSender < Confirmation > ,) -> Self { Self { message_rx , confirm_tx , attached : false , override_retcode : None , last_code : None } } }
};
}
