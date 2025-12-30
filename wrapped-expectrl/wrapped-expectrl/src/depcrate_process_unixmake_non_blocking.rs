// Generated macro for make_non_blocking (function)
macro_rules! Depcrate_process_unixmake_non_blocking {
() => {
// Module: crate::process::unix
// Provides: {"make_non_blocking"}
// Dependencies: {}
pub (crate) fn make_non_blocking (fd : RawFd , blocking : bool) -> Result < () > { use nix :: fcntl :: { fcntl , FcntlArg , OFlag } ; let opt = fcntl (fd , FcntlArg :: F_GETFL) . map_err (nix_error_to_io) ? ; let mut opt = OFlag :: from_bits_truncate (opt) ; opt . set (OFlag :: O_NONBLOCK , blocking) ; let _ = fcntl (fd , FcntlArg :: F_SETFL (opt)) . map_err (nix_error_to_io) ? ; Ok (()) }
};
}
