macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ioctl_read_bad {
    () => {
        deps!();
        # [doc = " Generates a wrapper function for a \"bad\" ioctl that reads data from the kernel."] # [doc = ""] # [doc = " The arguments to this macro are:"] # [doc = ""] # [doc = " * The function name"] # [doc = " * The ioctl request code"] # [doc = " * The data type passed by this ioctl"] # [doc = ""] # [doc = " The generated function has the following signature:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " pub unsafe fn FUNCTION_NAME(fd: libc::c_int, data: *mut DATA_TYPE) -> Result<libc::c_int>"] # [doc = " ```"] # [doc = ""] # [doc = " For a more in-depth explanation of ioctls, see [`::sys::ioctl`](sys/ioctl/index.html)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate nix;"] # [doc = " # #[cfg(linux_android)]"] # [doc = " ioctl_read_bad!(tcgets, libc::TCGETS, libc::termios);"] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! ioctl_read_bad { ($ (# [$ attr : meta]) * $ name : ident , $ nr : expr , $ ty : ty) => ($ (# [$ attr]) * pub unsafe fn $ name (fd : $ crate :: libc :: c_int , data : * mut $ ty) -> $ crate :: Result <$ crate :: libc :: c_int > { unsafe { convert_ioctl_res ! ($ crate :: libc :: ioctl (fd , $ nr as $ crate :: sys :: ioctl :: ioctl_num_type , data)) } }) }
    };
}

ioctl_read_bad!();