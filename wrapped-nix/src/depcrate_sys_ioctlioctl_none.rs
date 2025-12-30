// Generated macro for ioctl_none (macro)
macro_rules! Depcrate_sys_ioctlioctl_none {
() => {
// Module: crate::sys::ioctl
// Provides: {"ioctl_none"}
// Dependencies: {}
# [doc = " Generates a wrapper function for an ioctl that passes no data to the kernel."] # [doc = ""] # [doc = " The arguments to this macro are:"] # [doc = ""] # [doc = " * The function name"] # [doc = " * The ioctl identifier"] # [doc = " * The ioctl sequence number"] # [doc = ""] # [doc = " The generated function has the following signature:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " pub unsafe fn FUNCTION_NAME(fd: libc::c_int) -> Result<libc::c_int>"] # [doc = " ```"] # [doc = ""] # [doc = " For a more in-depth explanation of ioctls, see [`::sys::ioctl`](sys/ioctl/index.html)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " The `videodev2` driver on Linux defines the `log_status` `ioctl` as:"] # [doc = ""] # [doc = " ```C"] # [doc = " #define VIDIOC_LOG_STATUS         _IO('V', 70)"] # [doc = " ```"] # [doc = ""] # [doc = " This can be implemented in Rust like:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[macro_use] extern crate nix;"] # [doc = " ioctl_none!(log_status, b'V', 70);"] # [doc = " fn main() {}"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! ioctl_none { ($ (# [$ attr : meta]) * $ name : ident , $ ioty : expr , $ nr : expr) => ($ (# [$ attr]) * pub unsafe fn $ name (fd : $ crate :: libc :: c_int) -> $ crate :: Result <$ crate :: libc :: c_int > { unsafe { convert_ioctl_res ! ($ crate :: libc :: ioctl (fd , request_code_none ! ($ ioty , $ nr) as $ crate :: sys :: ioctl :: ioctl_num_type)) } }) }
};
}
