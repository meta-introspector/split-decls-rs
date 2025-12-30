// Generated macro for macro_634 (macro)
macro_rules! Depcrate_ffi_libcmacro_634 {
() => {
// Module: crate::ffi::libc
// Provides: {"macro_634"}
// Dependencies: {}
extern_c ! { # [doc = " The Objective-C runtime has several methods, usually with \"`copy`\" in"] # [doc = " their name, whose return value is allocated with C's `malloc` and"] # [doc = " deallocated with C's `free` method."] # [doc = ""] # [doc = " As such, `free` is actually also part of the Objective-C runtime."] # [doc = ""] # [doc = " We expose this instead of using [`libc::free`], to avoid having `libc`"] # [doc = " as a dependency."] # [doc = ""] # [doc = " [`libc::free`]: https://docs.rs/libc/latest/libc/fn.free.html"] pub fn free (p : * mut c_void) ; }
};
}
