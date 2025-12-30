// Generated macro for _XCTGetCurrentExceptionReasonWithFallback (function)
macro_rules! Depcrate_generated_XCTGetCurrentExceptionReasonWithFallback {
() => {
// Module: crate::generated
// Provides: {"_XCTGetCurrentExceptionReasonWithFallback"}
// Dependencies: {}
# [inline] pub (crate) extern "C-unwind" fn _XCTGetCurrentExceptionReasonWithFallback (fallback : Option < & NSString > ,) -> Retained < NSString > { extern "C-unwind" { fn _XCTGetCurrentExceptionReasonWithFallback (fallback : Option < & NSString >) -> * mut NSString ; } let ret = unsafe { _XCTGetCurrentExceptionReasonWithFallback (fallback) } ; unsafe { Retained :: retain_autoreleased (ret) } . expect ("function was marked as returning non-null, but actually returned NULL") }
};
}
