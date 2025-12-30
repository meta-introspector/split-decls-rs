// Generated macro for request_code_none (macro)
macro_rules! Depcrate_sys_ioctl_linuxrequest_code_none {
() => {
// Module: crate::sys::ioctl::linux
// Provides: {"request_code_none"}
// Dependencies: {}
# [doc = " Generate an ioctl request code for a command that passes no data."] # [doc = ""] # [doc = " This is equivalent to the `_IO()` macro exposed by the C ioctl API."] # [doc = ""] # [doc = " You should only use this macro directly if the `ioctl` you're working"] # [doc = " with is \"bad\" and you cannot use `ioctl_none!()` directly."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate nix;"] # [doc = " const KVMIO: u8 = 0xAE;"] # [doc = " ioctl_write_int_bad!(kvm_create_vm, request_code_none!(KVMIO, 0x03));"] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! request_code_none { ($ ty : expr , $ nr : expr) => { ioc ! ($ crate :: sys :: ioctl :: NONE , $ ty , $ nr , 0) } ; }
};
}
