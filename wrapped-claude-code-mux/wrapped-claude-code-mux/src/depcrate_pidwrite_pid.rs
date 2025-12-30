// Generated macro for write_pid (function)
macro_rules! Depcrate_pidwrite_pid {
() => {
// Module: crate::pid
// Provides: {"write_pid"}
// Dependencies: {}
# [doc = " Write the current process PID to the PID file"] pub fn write_pid () -> io :: Result < () > { let pid_file = get_pid_file () ; if let Some (parent) = pid_file . parent () { fs :: create_dir_all (parent) ? ; } let pid = std :: process :: id () ; fs :: write (& pid_file , pid . to_string ()) ? ; tracing :: info ! ("PID {} written to {:?}" , pid , pid_file) ; Ok (()) }
};
}
