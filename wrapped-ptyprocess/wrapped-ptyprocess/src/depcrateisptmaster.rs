// Generated macro for isptmaster (function)
macro_rules! Depcrateisptmaster {
() => {
// Module: crate
// Provides: {"isptmaster"}
// Dependencies: {}
# [cfg (target_os = "freebsd")] fn isptmaster (fd : RawFd) -> Result < bool > { use nix :: libc :: ioctl ; use nix :: libc :: TIOCPTMASTER ; match unsafe { ioctl (fd , TIOCPTMASTER as u64 , 0) } { 0 => Ok (true) , _ => Err (Error :: last ()) , } }
};
}
