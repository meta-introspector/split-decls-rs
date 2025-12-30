// Generated macro for get_pid_file (function)
macro_rules! Depcrate_pidget_pid_file {
() => {
// Module: crate::pid
// Provides: {"get_pid_file"}
// Dependencies: {}
# [doc = " Get the PID file path"] pub fn get_pid_file () -> PathBuf { let home = dirs :: home_dir () . unwrap_or_else (| | PathBuf :: from (".")) ; home . join (".claude-code-mux") . join ("ccm.pid") }
};
}
