// Generated macro for TraceRequest (enum)
macro_rules! Depcrate_shims_native_lib_trace_messagesTraceRequest {
() => {
// Module: crate::shims::native_lib::trace::messages
// Provides: {"TraceRequest"}
// Dependencies: {}
# [doc = " An IPC request sent by the child process to the parent."] # [doc = ""] # [doc = " The sender for this channel should live on the child process."] # [derive (serde :: Serialize , serde :: Deserialize , Debug , Clone)] pub enum TraceRequest { # [doc = " Requests that tracing begins. Following this being sent, the child must"] # [doc = " wait to receive a `Confirmation` on the respective channel and then"] # [doc = " `raise(SIGSTOP)`."] # [doc = ""] # [doc = " To avoid possible issues while allocating memory for IPC channels, ending"] # [doc = " the tracing is instead done via `raise(SIGUSR1)`."] StartFfi (StartFfiInfo) , # [doc = " Manually overrides the code that the supervisor will return upon exiting."] # [doc = " Once set, it is permanent. This can be called again to change the value."] # [doc = ""] # [doc = " After sending this, the child must wait to receive a `Confirmation`."] OverrideRetcode (i32) , }
};
}
