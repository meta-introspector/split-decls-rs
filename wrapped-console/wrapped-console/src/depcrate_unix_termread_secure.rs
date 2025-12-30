// Generated macro for read_secure (function)
macro_rules! Depcrate_unix_termread_secure {
() => {
// Module: crate::unix_term
// Provides: {"read_secure"}
// Dependencies: {}
pub (crate) fn read_secure () -> io :: Result < String > { let mut input = Input :: buffered () ? ; let mut termios = mem :: MaybeUninit :: uninit () ; c_result (| | unsafe { libc :: tcgetattr (input . as_raw_fd () , termios . as_mut_ptr ()) }) ? ; let mut termios = unsafe { termios . assume_init () } ; let original = termios ; termios . c_lflag &= ! libc :: ECHO ; c_result (| | unsafe { libc :: tcsetattr (input . as_raw_fd () , libc :: TCSAFLUSH , & termios) }) ? ; let mut rv = String :: new () ; let read_rv = input . read_line (& mut rv) ; c_result (| | unsafe { libc :: tcsetattr (input . as_raw_fd () , libc :: TCSAFLUSH , & original) }) ? ; read_rv . map (| _ | { let len = rv . trim_end_matches (& ['\r' , '\n'] [..]) . len () ; rv . truncate (len) ; rv }) }
};
}
