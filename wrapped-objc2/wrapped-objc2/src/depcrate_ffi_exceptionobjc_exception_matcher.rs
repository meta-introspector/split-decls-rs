// Generated macro for objc_exception_matcher (type)
macro_rules! Depcrate_ffi_exceptionobjc_exception_matcher {
() => {
// Module: crate::ffi::exception
// Provides: {"objc_exception_matcher"}
// Dependencies: {}
# [doc = " Remember that this is non-null!"] # [cfg (any (doc , all (target_vendor = "apple" , not (all (target_os = "macos" , target_arch = "x86")))))] type objc_exception_matcher = unsafe extern "C" fn (catch_type : * mut crate :: runtime :: AnyClass , exception : * mut AnyObject ,) -> core :: ffi :: c_int ;
};
}
