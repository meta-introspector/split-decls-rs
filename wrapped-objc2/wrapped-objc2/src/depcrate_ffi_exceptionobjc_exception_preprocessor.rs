// Generated macro for objc_exception_preprocessor (type)
macro_rules! Depcrate_ffi_exceptionobjc_exception_preprocessor {
() => {
// Module: crate::ffi::exception
// Provides: {"objc_exception_preprocessor"}
// Dependencies: {}
# [doc = " Remember that this is non-null!"] # [cfg (any (doc , all (target_vendor = "apple" , not (all (target_os = "macos" , target_arch = "x86")))))] type objc_exception_preprocessor = unsafe extern "C" fn (exception : * mut AnyObject) -> * mut AnyObject ;
};
}
