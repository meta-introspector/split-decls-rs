// Generated macro for CommandState (enum)
macro_rules! Depcrate_utils_execCommandState {
() => {
// Module: crate::utils::exec
// Provides: {"CommandState"}
// Dependencies: {}
enum CommandState < 'a > { Cached (CommandOutput) , Deferred { process : Option < Result < Child , std :: io :: Error > > , command : & 'a mut BootstrapCommand , stdout : OutputMode , stderr : OutputMode , executed_at : & 'a Location < 'a > , fingerprint : CommandFingerprint , start_time : Instant , # [cfg (feature = "tracing")] _span_guard : tracing :: span :: EnteredSpan , } , }
};
}
