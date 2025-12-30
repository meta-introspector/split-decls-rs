// Generated macro for request_code_write_int (macro)
macro_rules! Depcrate_sys_ioctl_bsdrequest_code_write_int {
() => {
// Module: crate::sys::ioctl::bsd
// Provides: {"request_code_write_int"}
// Dependencies: {}
# [doc = " Generate an ioctl request code for a command that passes an integer"] # [doc = ""] # [doc = " This is equivalent to the `_IOWINT()` macro exposed by the C ioctl API."] # [doc = ""] # [doc = " You should only use this macro directly if the `ioctl` you're working"] # [doc = " with is \"bad\" and you cannot use `ioctl_write_int!()` directly."] # [macro_export (local_inner_macros)] macro_rules ! request_code_write_int { ($ g : expr , $ n : expr) => { ioc ! ($ crate :: sys :: ioctl :: VOID , $ g , $ n , :: std :: mem :: size_of ::<$ crate :: libc :: c_int > ()) } ; }
};
}
