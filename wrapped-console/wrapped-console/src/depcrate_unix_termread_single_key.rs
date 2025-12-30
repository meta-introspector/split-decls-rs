// Generated macro for read_single_key (function)
macro_rules! Depcrate_unix_termread_single_key {
() => {
// Module: crate::unix_term
// Provides: {"read_single_key"}
// Dependencies: {}
pub (crate) fn read_single_key (ctrlc_key : bool) -> io :: Result < Key > { let input = Input :: unbuffered () ? ; let mut termios = core :: mem :: MaybeUninit :: uninit () ; c_result (| | unsafe { libc :: tcgetattr (input . as_raw_fd () , termios . as_mut_ptr ()) }) ? ; let mut termios = unsafe { termios . assume_init () } ; let original = termios ; make_raw (& mut termios) ; termios . c_oflag = original . c_oflag ; c_result (| | unsafe { libc :: tcsetattr (input . as_raw_fd () , libc :: TCSADRAIN , & termios) }) ? ; let rv = read_single_key_impl (input . as_raw_fd ()) ; c_result (| | unsafe { libc :: tcsetattr (input . as_raw_fd () , libc :: TCSADRAIN , & original) }) ? ; if let Err (ref err) = rv { if err . kind () == io :: ErrorKind :: Interrupted { if ! ctrlc_key { unsafe { libc :: raise (libc :: SIGINT) ; } } else { return Ok (Key :: CtrlC) ; } } } rv }
};
}
