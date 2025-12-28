macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ioctl_none_bad {
    () => {
        deps!();
        # [doc = " Generates a wrapper function for a \"bad\" ioctl that passes no data to the kernel."] # [doc = ""] # [doc = " The arguments to this macro are:"] # [doc = ""] # [doc = " * The function name"] # [doc = " * The ioctl request code"] # [doc = ""] # [doc = " The generated function has the following signature:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " pub unsafe fn FUNCTION_NAME(fd: libc::c_int) -> Result<libc::c_int>"] # [doc = " ```"] # [doc = ""] # [doc = " For a more in-depth explanation of ioctls, see [`::sys::ioctl`](sys/ioctl/index.html)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[macro_use] extern crate nix;"] # [doc = " # use libc::TIOCNXCL;"] # [doc = " # use std::fs::File;"] # [doc = " # use std::os::unix::io::AsRawFd;"] # [doc = " ioctl_none_bad!(tiocnxcl, TIOCNXCL);"] # [doc = " fn main() {"] # [doc = "     let file = File::open(\"/dev/ttyUSB0\").unwrap();"] # [doc = "     unsafe { tiocnxcl(file.as_raw_fd()) }.unwrap();"] # [doc = " }"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! ioctl_none_bad { ($ (# [$ attr : meta]) * $ name : ident , $ nr : expr) => ($ (# [$ attr]) * pub unsafe fn $ name (fd : $ crate :: libc :: c_int) -> $ crate :: Result <$ crate :: libc :: c_int > { unsafe { convert_ioctl_res ! ($ crate :: libc :: ioctl (fd , $ nr as $ crate :: sys :: ioctl :: ioctl_num_type)) } }) }
    };
}

ioctl_none_bad!()