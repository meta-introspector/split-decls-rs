// Generated macro for macro_630 (macro)
macro_rules! Depcrate_ffi_exceptionmacro_630 {
() => {
// Module: crate::ffi::exception
// Provides: {"macro_630"}
// Dependencies: {}
extern_c_unwind ! { # [doc = " See [`objc-exception.h`]."] # [doc = ""] # [doc = " [`objc-exception.h`]: https://github.com/apple-oss-distributions/objc4/blob/objc4-818.2/runtime/objc-exception.h"] # [cold] pub fn objc_exception_throw (exception : * mut AnyObject) -> !; # [cfg (all (target_vendor = "apple" , not (feature = "gnustep-1-7") , not (all (target_os = "macos" , target_arch = "x86"))))] # [cold] pub fn objc_exception_rethrow () -> !; # [cfg (feature = "gnustep-1-7")] # [cold] pub fn objc_exception_rethrow (exc_buf : * mut core :: ffi :: c_void) -> !; }
};
}
