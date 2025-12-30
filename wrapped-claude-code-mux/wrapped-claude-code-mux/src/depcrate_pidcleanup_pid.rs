// Generated macro for cleanup_pid (function)
macro_rules! Depcrate_pidcleanup_pid {
() => {
// Module: crate::pid
// Provides: {"cleanup_pid"}
// Dependencies: {}
# [doc = " Remove the PID file"] pub fn cleanup_pid () -> io :: Result < () > { let pid_file = get_pid_file () ; if pid_file . exists () { fs :: remove_file (& pid_file) ? ; tracing :: info ! ("PID file removed: {:?}" , pid_file) ; } Ok (()) }
};
}
