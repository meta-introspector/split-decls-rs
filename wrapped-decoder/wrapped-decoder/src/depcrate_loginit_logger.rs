// Generated macro for init_logger (function)
macro_rules! Depcrate_loginit_logger {
() => {
// Module: crate::log
// Provides: {"init_logger"}
// Dependencies: {}
# [doc = " Initializes a `log` sink that handles defmt frames."] # [doc = ""] # [doc = " Defmt frames will be printed to stdout, other logs to stderr."] # [doc = ""] # [doc = " The caller has to provide a `should_log` closure that determines whether a log record should be"] # [doc = " printed."] pub fn init_logger (formatter : Formatter , host_formatter : HostFormatter , logger_type : DefmtLoggerType , should_log : impl Fn (& Metadata) -> bool + Sync + Send + 'static ,) { let logger : Box < dyn Log > = match logger_type { DefmtLoggerType :: Stdout => StdoutLogger :: new (formatter , host_formatter , should_log) , DefmtLoggerType :: Json => { JsonLogger :: print_schema_version () ; JsonLogger :: new (formatter , host_formatter , should_log) } } ; alterable_logger :: set_boxed_logger (logger) ; alterable_logger :: set_max_level (LevelFilter :: Trace) ; }
};
}
