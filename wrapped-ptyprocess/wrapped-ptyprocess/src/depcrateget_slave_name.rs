// Generated macro for get_slave_name (function)
macro_rules! Depcrateget_slave_name {
() => {
// Module: crate
// Provides: {"get_slave_name"}
// Dependencies: {}
# [doc = " Getting a slave name on darvin platform"] # [doc = " https://blog.tarq.io/ptsname-on-osx-with-rust/"] # [cfg (target_os = "macos")] fn get_slave_name (fd : & PtyMaster) -> Result < String > { use nix :: libc :: ioctl ; use nix :: libc :: TIOCPTYGNAME ; use std :: ffi :: CStr ; use std :: os :: raw :: c_char ; use std :: os :: unix :: prelude :: AsRawFd ; let mut buf : [c_char ; 128] = [0 ; 128] ; let fd = fd . as_raw_fd () ; match unsafe { ioctl (fd , TIOCPTYGNAME as u64 , & mut buf) } { 0 => { let string = unsafe { CStr :: from_ptr (buf . as_ptr ()) } . to_string_lossy () . into_owned () ; return Ok (string) ; } _ => Err (Error :: last ()) , } }
};
}
