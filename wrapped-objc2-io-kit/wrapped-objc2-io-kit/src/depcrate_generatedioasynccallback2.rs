// Generated macro for IOAsyncCallback2 (type)
macro_rules! Depcrate_generatedIOAsyncCallback2 {
() => {
// Module: crate::generated
// Provides: {"IOAsyncCallback2"}
// Dependencies: {}
# [doc = " standard callback function for asynchronous I/O requests with"] # [doc = " two extra arguments beyond a refcon and result code."] # [doc = ""] # [doc = " Parameter `refcon`: The refcon passed into the original I/O request"] # [doc = ""] # [doc = " Parameter `result`: The result of the I/O operation"] # [doc = ""] # [doc = " Parameter `arg0`: Extra argument"] # [doc = ""] # [doc = " Parameter `arg1`: Extra argument"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/ioasynccallback2?language=objc)"] pub type IOAsyncCallback2 = Option < unsafe extern "C-unwind" fn (* mut c_void , IOReturn , * mut c_void , * mut c_void) > ;
};
}
