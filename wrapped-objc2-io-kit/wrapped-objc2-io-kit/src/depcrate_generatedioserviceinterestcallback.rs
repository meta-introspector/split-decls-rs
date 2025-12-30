// Generated macro for IOServiceInterestCallback (type)
macro_rules! Depcrate_generatedIOServiceInterestCallback {
() => {
// Module: crate::generated
// Provides: {"IOServiceInterestCallback"}
// Dependencies: {}
# [doc = " Callback function to be notified of changes in state of an IOService."] # [doc = ""] # [doc = " Parameter `refcon`: The refcon passed when the notification was installed."] # [doc = ""] # [doc = " Parameter `service`: The IOService whose state has changed."] # [doc = ""] # [doc = " Parameter `messageType`: A messageType enum, defined by IOKit/IOMessage.h or by the IOService's family."] # [doc = ""] # [doc = " Parameter `messageArgument`: An argument for the message, dependent on the messageType.  If the message data is larger than sizeof(void*), then messageArgument contains a pointer to the message data; otherwise, messageArgument contains the message data."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/ioserviceinterestcallback?language=objc)"] # [cfg (feature = "libc")] pub type IOServiceInterestCallback = Option < unsafe extern "C-unwind" fn (* mut c_void , io_service_t , u32 , * mut c_void) > ;
};
}
