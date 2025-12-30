// Generated macro for IOServiceMatchingCallback (type)
macro_rules! Depcrate_generatedIOServiceMatchingCallback {
() => {
// Module: crate::generated
// Provides: {"IOServiceMatchingCallback"}
// Dependencies: {}
# [doc = " Callback function to be notified of IOService publication."] # [doc = ""] # [doc = " Parameter `refcon`: The refcon passed when the notification was installed."] # [doc = ""] # [doc = " Parameter `iterator`: The notification iterator which now has new objects."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/ioservicematchingcallback?language=objc)"] # [cfg (feature = "libc")] pub type IOServiceMatchingCallback = Option < unsafe extern "C-unwind" fn (* mut c_void , io_iterator_t) > ;
};
}
