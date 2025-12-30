// Generated macro for make_controlling_tty (function)
macro_rules! Depcratemake_controlling_tty {
() => {
// Module: crate
// Provides: {"make_controlling_tty"}
// Dependencies: {}
fn make_controlling_tty (ptm : & Master) -> Result < () > { # [cfg (not (any (target_os = "freebsd" , target_os = "macos")))] { let pts_name = ptm . get_slave_name () ? ; let fd = open ("/dev/tty" , OFlag :: O_RDWR | OFlag :: O_NOCTTY , Mode :: empty ()) ; match fd { Ok (fd) => { close (fd) ? ; } Err (Error :: ENXIO) => { } Err (err) => return Err (err) , } setsid () ? ; let fd = open ("/dev/tty" , OFlag :: O_RDWR | OFlag :: O_NOCTTY , Mode :: empty ()) ; match fd { Err (Error :: ENXIO) => { } Ok (fd) => { close (fd) ? ; return Err (Error :: ENOTSUP) ; } Err (_) => return Err (Error :: ENOTSUP) , } let fd = open (pts_name . as_str () , OFlag :: O_RDWR , Mode :: empty ()) ? ; close (fd) ? ; let fd = open ("/dev/tty" , OFlag :: O_WRONLY , Mode :: empty ()) ? ; close (fd) ? ; } # [cfg (any (target_os = "freebsd" , target_os = "macos"))] { let pts_fd = ptm . get_slave_fd () ? ; setsid () ? ; use nix :: libc :: ioctl ; use nix :: libc :: TIOCSCTTY ; match unsafe { ioctl (pts_fd , TIOCSCTTY as u64 , 0) } { 0 => { } _ => return Err (Error :: last ()) , } } Ok (()) }
};
}
