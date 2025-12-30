// Generated macro for throw (function)
macro_rules! Depcrate_exceptionthrow {
() => {
// Module: crate::exception
// Provides: {"throw"}
// Dependencies: {}
# [doc = " Throws an Objective-C exception."] # [doc = ""] # [doc = " This is the Objective-C equivalent of Rust's [`panic!`]."] # [doc = ""] # [doc = " This unwinds from Objective-C, and the exception should be caught using an"] # [doc = " Objective-C exception handler like [`catch`]. It _may_ be caught by"] # [doc = " [`catch_unwind`], though the error message is unlikely to be great."] # [doc = ""] # [doc = " [`catch_unwind`]: std::panic::catch_unwind"] # [inline] # [cfg (feature = "exception")] pub fn throw (exception : Retained < Exception >) -> ! { let ptr : * const AnyObject = & exception . 0 ; let ptr = ptr as * mut AnyObject ; unsafe { ffi :: objc_exception_throw (ptr) } }
};
}
