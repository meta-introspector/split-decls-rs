// Generated macro for init (function)
macro_rules! Depcrate_logginginit {
() => {
// Module: crate::logging
// Provides: {"init"}
// Dependencies: {}
pub unsafe fn init () { log :: set_logger (& KERNEL_LOGGER) . expect ("Can't initialize logger") ; let log_level : Option < & 'static str > = option_env ! ("HERMIT_LOG_LEVEL_FILTER") ; let mut max_level = LevelFilter :: Info ; if let Some (log_level) = log_level { max_level = if log_level . eq_ignore_ascii_case ("off") { LevelFilter :: Off } else if log_level . eq_ignore_ascii_case ("error") { LevelFilter :: Error } else if log_level . eq_ignore_ascii_case ("warn") { LevelFilter :: Warn } else if log_level . eq_ignore_ascii_case ("info") { LevelFilter :: Info } else if log_level . eq_ignore_ascii_case ("debug") { LevelFilter :: Debug } else if log_level . eq_ignore_ascii_case ("trace") { LevelFilter :: Trace } else { error ! ("Could not parse HERMIT_LOG_LEVEL_FILTER, falling back to `info`.") ; LevelFilter :: Info } ; } log :: set_max_level (max_level) ; }
};
}
