// Generated macro for IOAsyncCallback (type)
macro_rules! Depcrate_generatedIOAsyncCallback {
() => {
// Module: crate::generated
// Provides: {"IOAsyncCallback"}
// Dependencies: {}
# [doc = " standard callback function for asynchronous I/O requests with"] # [doc = " lots of extra arguments beyond a refcon and result code."] # [doc = ""] # [doc = " Parameter `refcon`: The refcon passed into the original I/O request"] # [doc = ""] # [doc = " Parameter `result`: The result of the I/O operation"] # [doc = ""] # [doc = " Parameter `args`: Array of extra arguments"] # [doc = ""] # [doc = " Parameter `numArgs`: Number of extra arguments"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/ioasynccallback?language=objc)"] pub type IOAsyncCallback = Option < unsafe extern "C-unwind" fn (* mut c_void , IOReturn , * mut * mut c_void , u32) > ;
};
}
