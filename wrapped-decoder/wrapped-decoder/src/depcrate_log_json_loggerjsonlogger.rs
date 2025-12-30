// Generated macro for JsonLogger (struct)
macro_rules! Depcrate_log_json_loggerJsonLogger {
() => {
// Module: crate::log::json_logger
// Provides: {"JsonLogger"}
// Dependencies: {}
pub (crate) struct JsonLogger { should_log : Box < dyn Fn (& Metadata) -> bool + Sync + Send > , host_logger : StdoutLogger , }
};
}
