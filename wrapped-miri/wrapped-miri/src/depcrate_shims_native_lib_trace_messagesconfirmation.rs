// Generated macro for Confirmation (struct)
macro_rules! Depcrate_shims_native_lib_trace_messagesConfirmation {
() => {
// Module: crate::shims::native_lib::trace::messages
// Provides: {"Confirmation"}
// Dependencies: {}
# [doc = " A marker type confirming that the supervisor has received the request to begin"] # [doc = " tracing and is now waiting for a `SIGSTOP`."] # [doc = ""] # [doc = " The sender for this channel should live on the parent process."] # [derive (serde :: Serialize , serde :: Deserialize , Debug)] pub struct Confirmation ;
};
}
