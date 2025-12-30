// Generated macro for IOAsyncCallback0 (type)
macro_rules! Depcrate_generatedIOAsyncCallback0 {
() => {
// Module: crate::generated
// Provides: {"IOAsyncCallback0"}
// Dependencies: {}
# [doc = " standard callback function for asynchronous I/O requests with"] # [doc = " no extra arguments beyond a refcon and result code."] # [doc = ""] # [doc = " Parameter `refcon`: The refcon passed into the original I/O request"] # [doc = ""] # [doc = " Parameter `result`: The result of the I/O operation"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/ioasynccallback0?language=objc)"] pub type IOAsyncCallback0 = Option < unsafe extern "C-unwind" fn (* mut c_void , IOReturn) > ;
};
}
