// Generated macro for request_code_readwrite (macro)
macro_rules! Depcrate_sys_ioctl_bsdrequest_code_readwrite {
() => {
// Module: crate::sys::ioctl::bsd
// Provides: {"request_code_readwrite"}
// Dependencies: {}
# [doc = " Generate an ioctl request code for a command that reads and writes."] # [doc = ""] # [doc = " This is equivalent to the `_IOWR()` macro exposed by the C ioctl API."] # [doc = ""] # [doc = " You should only use this macro directly if the `ioctl` you're working"] # [doc = " with is \"bad\" and you cannot use `ioctl_readwrite!()` directly."] # [macro_export (local_inner_macros)] macro_rules ! request_code_readwrite { ($ g : expr , $ n : expr , $ len : expr) => { ioc ! ($ crate :: sys :: ioctl :: INOUT , $ g , $ n , $ len) } ; }
};
}
