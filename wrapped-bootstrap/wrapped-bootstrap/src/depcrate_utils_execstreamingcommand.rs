// Generated macro for StreamingCommand (struct)
macro_rules! Depcrate_utils_execStreamingCommand {
() => {
// Module: crate::utils::exec
// Provides: {"StreamingCommand"}
// Dependencies: {}
pub struct StreamingCommand { child : Child , pub stdout : Option < ChildStdout > , pub stderr : Option < ChildStderr > , fingerprint : CommandFingerprint , start_time : Instant , # [cfg (feature = "tracing")] _span_guard : tracing :: span :: EnteredSpan , }
};
}
