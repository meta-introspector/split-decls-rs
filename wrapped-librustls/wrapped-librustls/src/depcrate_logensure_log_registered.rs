// Generated macro for ensure_log_registered (function)
macro_rules! Depcrate_logensure_log_registered {
() => {
// Module: crate::log
// Provides: {"ensure_log_registered"}
// Dependencies: {}
# [cfg (not (feature = "no_log_capture"))] pub (crate) fn ensure_log_registered () { log :: set_logger (& Logger { }) . ok () ; log :: set_max_level (log :: LevelFilter :: Debug) }
};
}
