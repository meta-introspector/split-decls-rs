// Generated macro for sysinfo (function)
macro_rules! Depcrate_sys_sysinfosysinfo {
() => {
// Module: crate::sys::sysinfo
// Provides: {"sysinfo"}
// Dependencies: {}
# [doc = " Returns system information."] # [doc = ""] # [doc = " [See `sysinfo(2)`](https://man7.org/linux/man-pages/man2/sysinfo.2.html)."] pub fn sysinfo () -> Result < SysInfo > { let mut info = mem :: MaybeUninit :: uninit () ; let res = unsafe { libc :: sysinfo (info . as_mut_ptr ()) } ; Errno :: result (res) . map (| _ | unsafe { SysInfo (info . assume_init ()) }) }
};
}
