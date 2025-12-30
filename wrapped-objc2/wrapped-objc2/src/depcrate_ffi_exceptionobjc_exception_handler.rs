// Generated macro for objc_exception_handler (type)
macro_rules! Depcrate_ffi_exceptionobjc_exception_handler {
() => {
// Module: crate::ffi::exception
// Provides: {"objc_exception_handler"}
// Dependencies: {}
# [doc = " Remember that this is non-null!"] # [cfg (any (doc , all (target_vendor = "apple" , target_os = "macos" , not (target_arch = "x86"))))] type objc_exception_handler = unsafe extern "C" fn (unused : * mut AnyObject , context : * mut core :: ffi :: c_void) ;
};
}
