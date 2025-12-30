// Generated macro for objc_uncaught_exception_handler (type)
macro_rules! Depcrate_ffi_exceptionobjc_uncaught_exception_handler {
() => {
// Module: crate::ffi::exception
// Provides: {"objc_uncaught_exception_handler"}
// Dependencies: {}
# [cfg (feature = "unstable-objfw")] type objc_uncaught_exception_handler = Option < unsafe extern "C" fn (exception : * mut AnyObject) > ;
};
}
