// Generated macro for set_echo (function)
macro_rules! Depcrateset_echo {
() => {
// Module: crate
// Provides: {"set_echo"}
// Dependencies: {}
fn set_echo (fd : RawFd , on : bool) -> Result < () > { let mut flags = termios :: tcgetattr (fd) ? ; match on { true => flags . local_flags |= termios :: LocalFlags :: ECHO , false => flags . local_flags &= ! termios :: LocalFlags :: ECHO , } termios :: tcsetattr (fd , termios :: SetArg :: TCSANOW , & flags) ? ; Ok (()) }
};
}
