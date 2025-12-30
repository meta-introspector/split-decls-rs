// Generated macro for imp (module)
macro_rules! Depcrate_stdioimp {
() => {
// Module: crate::stdio
// Provides: {"imp"}
// Dependencies: {}
# [cfg (unix)] mod imp { use super :: Stdio ; use libc :: { STDIN_FILENO , STDOUT_FILENO , close , dup , dup2 } ; use std :: { fs :: File , io :: Error , os :: fd :: AsRawFd } ; pub const IN_DEVICE : & str = "/dev/tty" ; pub const OUT_DEVICE : & str = "/dev/tty" ; pub const NULL_DEVICE : & str = "/dev/null" ; # [doc = " Restores previous stdio when dropped."] pub struct ReplacementGuard { std_fileno : i32 , previous : i32 , } impl ReplacementGuard { pub (super) fn new (stdio : Stdio , replacement : & mut File) -> Result < ReplacementGuard , Error > { let std_fileno = match stdio { Stdio :: Stdin => STDIN_FILENO , Stdio :: Stdout => STDOUT_FILENO , } ; let previous ; unsafe { previous = dup (std_fileno) ; if previous == - 1 { return Err (std :: io :: Error :: last_os_error ()) ; } if dup2 (replacement . as_raw_fd () , std_fileno) == - 1 { return Err (std :: io :: Error :: last_os_error ()) ; } } Ok (ReplacementGuard { previous , std_fileno , }) } } impl Drop for ReplacementGuard { fn drop (& mut self) { unsafe { dup2 (self . previous , self . std_fileno) ; close (self . previous) ; } } } }
};
}
