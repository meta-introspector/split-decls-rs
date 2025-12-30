// Generated macro for redirect_std_streams (function)
macro_rules! Depcrateredirect_std_streams {
() => {
// Module: crate
// Provides: {"redirect_std_streams"}
// Dependencies: {}
fn redirect_std_streams (fd : RawFd) -> Result < () > { close (STDIN_FILENO) ? ; close (STDOUT_FILENO) ? ; close (STDERR_FILENO) ? ; dup2 (fd , STDIN_FILENO) ? ; dup2 (fd , STDOUT_FILENO) ? ; dup2 (fd , STDERR_FILENO) ? ; Ok (()) }
};
}
