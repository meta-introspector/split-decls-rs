macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ioctl_write_int_bad {
    () => {
        deps!();
        # [doc = " Generates a wrapper function for a \"bad\" ioctl that writes an integer to the kernel."] # [doc = ""] # [doc = " The arguments to this macro are:"] # [doc = ""] # [doc = " * The function name"] # [doc = " * The ioctl request code"] # [doc = ""] # [doc = " The generated function has the following signature:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " pub unsafe fn FUNCTION_NAME(fd: libc::c_int, data: libc::c_int) -> Result<libc::c_int>"] # [doc = " ```"] # [doc = ""] # [doc = " For a more in-depth explanation of ioctls, see [`::sys::ioctl`](sys/ioctl/index.html)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate nix;"] # [doc = " # #[cfg(linux_android)]"] # [doc = " ioctl_write_int_bad!(tcsbrk, libc::TCSBRK);"] # [doc = " # fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[macro_use] extern crate nix;"] # [doc = " const KVMIO: u8 = 0xAE;"] # [doc = " ioctl_write_int_bad!(kvm_create_vm, request_code_none!(KVMIO, 0x03));"] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! ioctl_write_int_bad { ($ (# [$ attr : meta]) * $ name : ident , $ nr : expr) => ($ (# [$ attr]) * pub unsafe fn $ name (fd : $ crate :: libc :: c_int , data : $ crate :: libc :: c_int) -> $ crate :: Result <$ crate :: libc :: c_int > { unsafe { convert_ioctl_res ! ($ crate :: libc :: ioctl (fd , $ nr as $ crate :: sys :: ioctl :: ioctl_num_type , data)) } }) }
    };
}

ioctl_write_int_bad!()