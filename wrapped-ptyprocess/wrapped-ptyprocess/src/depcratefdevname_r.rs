// Generated macro for fdevname_r (function)
macro_rules! Depcratefdevname_r {
() => {
// Module: crate
// Provides: {"fdevname_r"}
// Dependencies: {}
# [cfg (target_os = "freebsd")] fn fdevname_r (fd : RawFd , buf : & mut [std :: os :: raw :: c_char]) -> Result < () > { use nix :: libc :: { ioctl , FIODGNAME } ; nix :: ioctl_read_bad ! (_ioctl_fiodgname , FIODGNAME , fiodgname_arg) ; let mut fgn = fiodgname_arg { len : buf . len () as i32 , buf : buf . as_mut_ptr () as * mut :: std :: os :: raw :: c_void , } ; let _ = unsafe { _ioctl_fiodgname (fd , & mut fgn) } ? ; Ok (()) }
};
}
