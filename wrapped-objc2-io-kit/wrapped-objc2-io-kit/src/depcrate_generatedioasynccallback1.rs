// Generated macro for IOAsyncCallback1 (type)
macro_rules! Depcrate_generatedIOAsyncCallback1 {
() => {
// Module: crate::generated
// Provides: {"IOAsyncCallback1"}
// Dependencies: {}
# [doc = " standard callback function for asynchronous I/O requests with"] # [doc = " one extra argument beyond a refcon and result code."] # [doc = " This is often a count of the number of bytes transferred"] # [doc = ""] # [doc = " Parameter `refcon`: The refcon passed into the original I/O request"] # [doc = ""] # [doc = " Parameter `result`: The result of the I/O operation"] # [doc = ""] # [doc = " Parameter `arg0`: Extra argument"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/ioasynccallback1?language=objc)"] pub type IOAsyncCallback1 = Option < unsafe extern "C-unwind" fn (* mut c_void , IOReturn , * mut c_void) > ;
};
}
