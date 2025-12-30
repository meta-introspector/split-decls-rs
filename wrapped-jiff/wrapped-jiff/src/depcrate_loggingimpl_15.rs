// Generated macro for impl_15 (impl)
macro_rules! Depcrate_loggingimpl_15 {
() => {
// Module: crate::logging
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (all (test))] impl Logger { # [doc = " Create a new logger that logs to stderr and initialize it as the"] # [doc = " global logger. If there was a problem setting the logger, then an"] # [doc = " error is returned."] pub (crate) fn init () -> Result < () , crate :: Error > { # [cfg (all (feature = "std" , feature = "logging"))] { log :: set_logger (LOGGER) . map_err (crate :: Error :: adhoc) ? ; log :: set_max_level (log :: LevelFilter :: Trace) ; Ok (()) } # [cfg (not (all (feature = "std" , feature = "logging")))] { Ok (()) } } }
};
}
