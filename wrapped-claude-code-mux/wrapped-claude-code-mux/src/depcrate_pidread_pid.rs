// Generated macro for read_pid (function)
macro_rules! Depcrate_pidread_pid {
() => {
// Module: crate::pid
// Provides: {"read_pid"}
// Dependencies: {}
# [doc = " Read the PID from the PID file"] pub fn read_pid () -> io :: Result < u32 > { let pid_file = get_pid_file () ; let pid_str = fs :: read_to_string (& pid_file) ? ; pid_str . trim () . parse :: < u32 > () . map_err (| e | io :: Error :: new (ErrorKind :: InvalidData , e)) }
};
}
