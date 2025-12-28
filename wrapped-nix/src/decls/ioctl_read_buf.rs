macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ioctl_read_buf {
    () => {
        deps!();
        # [doc = " Generates a wrapper function for an ioctl that reads an array of elements from the kernel."] # [doc = ""] # [doc = " The arguments to this macro are:"] # [doc = ""] # [doc = " * The function name"] # [doc = " * The ioctl identifier"] # [doc = " * The ioctl sequence number"] # [doc = " * The data type passed by this ioctl"] # [doc = ""] # [doc = " The generated function has the following signature:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " pub unsafe fn FUNCTION_NAME(fd: libc::c_int, data: &mut [DATA_TYPE]) -> Result<libc::c_int>"] # [doc = " ```"] # [doc = ""] # [doc = " For a more in-depth explanation of ioctls, see [`::sys::ioctl`](sys/ioctl/index.html)."] # [macro_export (local_inner_macros)] macro_rules ! ioctl_read_buf { ($ (# [$ attr : meta]) * $ name : ident , $ ioty : expr , $ nr : expr , $ ty : ty) => ($ (# [$ attr]) * pub unsafe fn $ name (fd : $ crate :: libc :: c_int , data : & mut [$ ty]) -> $ crate :: Result <$ crate :: libc :: c_int > { unsafe { convert_ioctl_res ! ($ crate :: libc :: ioctl (fd , request_code_read ! ($ ioty , $ nr , :: std :: mem :: size_of_val (data)) as $ crate :: sys :: ioctl :: ioctl_num_type , data . as_mut_ptr ())) } }) }
    };
}

ioctl_read_buf!();