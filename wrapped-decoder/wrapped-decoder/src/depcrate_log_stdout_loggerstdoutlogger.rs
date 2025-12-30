// Generated macro for StdoutLogger (struct)
macro_rules! Depcrate_log_stdout_loggerStdoutLogger {
() => {
// Module: crate::log::stdout_logger
// Provides: {"StdoutLogger"}
// Dependencies: {}
pub (crate) struct StdoutLogger { formatter : Formatter , host_formatter : HostFormatter , should_log : Box < dyn Fn (& Metadata) -> bool + Sync + Send > , }
};
}
