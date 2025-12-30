// Generated macro for set_raw (function)
macro_rules! Depcrateset_raw {
() => {
// Module: crate
// Provides: {"set_raw"}
// Dependencies: {}
pub fn set_raw (fd : RawFd) -> Result < () > { let mut flags = termios :: tcgetattr (fd) ? ; # [cfg (not (target_os = "macos"))] { termios :: cfmakeraw (& mut flags) ; } # [cfg (target_os = "macos")] { use nix :: libc :: { VMIN , VTIME } ; use termios :: ControlFlags ; use termios :: InputFlags ; use termios :: LocalFlags ; use termios :: OutputFlags ; flags . input_flags &= ! (InputFlags :: BRKINT | InputFlags :: ICRNL | InputFlags :: INPCK | InputFlags :: ISTRIP | InputFlags :: IXON) ; flags . output_flags &= ! OutputFlags :: OPOST ; flags . control_flags &= ! (ControlFlags :: CSIZE | ControlFlags :: PARENB) ; flags . control_flags |= ControlFlags :: CS8 ; flags . local_flags &= ! (LocalFlags :: ECHO | LocalFlags :: ICANON | LocalFlags :: IEXTEN | LocalFlags :: ISIG) ; flags . control_chars [VMIN] = 1 ; flags . control_chars [VTIME] = 0 ; } termios :: tcsetattr (fd , termios :: SetArg :: TCSANOW , & flags) ? ; Ok (()) }
};
}
