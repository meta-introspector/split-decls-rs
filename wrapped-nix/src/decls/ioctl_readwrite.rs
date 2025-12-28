macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ioctl_readwrite {
    () => {
        deps!();
        # [doc = " Generates a wrapper function for an ioctl that reads and writes data to the kernel."] # [doc = ""] # [doc = " The arguments to this macro are:"] # [doc = ""] # [doc = " * The function name"] # [doc = " * The ioctl identifier"] # [doc = " * The ioctl sequence number"] # [doc = " * The data type passed by this ioctl"] # [doc = ""] # [doc = " The generated function has the following signature:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " pub unsafe fn FUNCTION_NAME(fd: libc::c_int, data: *mut DATA_TYPE) -> Result<libc::c_int>"] # [doc = " ```"] # [doc = ""] # [doc = " For a more in-depth explanation of ioctls, see [`::sys::ioctl`](sys/ioctl/index.html)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate nix;"] # [doc = " # pub struct v4l2_audio {}"] # [doc = " ioctl_readwrite!(enum_audio, b'V', 65, v4l2_audio);"] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! ioctl_readwrite { ($ (# [$ attr : meta]) * $ name : ident , $ ioty : expr , $ nr : expr , $ ty : ty) => ($ (# [$ attr]) * pub unsafe fn $ name (fd : $ crate :: libc :: c_int , data : * mut $ ty) -> $ crate :: Result <$ crate :: libc :: c_int > { let ioty = $ ioty ; let nr = $ nr ; unsafe { convert_ioctl_res ! ($ crate :: libc :: ioctl (fd , request_code_readwrite ! (ioty , nr , :: std :: mem :: size_of ::<$ ty > ()) as $ crate :: sys :: ioctl :: ioctl_num_type , data)) } }) }
    };
}

ioctl_readwrite!();