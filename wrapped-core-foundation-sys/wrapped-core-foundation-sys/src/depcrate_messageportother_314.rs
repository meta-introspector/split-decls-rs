// Generated macro for other_314 (other)
macro_rules! Depcrate_messageportother_314 {
() => {
// Module: crate::messageport
// Provides: {"other_314"}
// Dependencies: {}
unsafe extern "C" { pub fn CFMessagePortCreateLocal (allocator : CFAllocatorRef , name : CFStringRef , callout : CFMessagePortCallBack , context : * const CFMessagePortContext , shouldFreeInfo : * mut Boolean ,) -> CFMessagePortRef ; pub fn CFMessagePortCreateRemote (allocator : CFAllocatorRef , name : CFStringRef ,) -> CFMessagePortRef ; pub fn CFMessagePortCreateRunLoopSource (allocator : CFAllocatorRef , local : CFMessagePortRef , order : CFIndex ,) -> CFRunLoopSourceRef ; pub fn CFMessagePortSetInvalidationCallBack (ms : CFMessagePortRef , callout : CFMessagePortInvalidationCallBack ,) ; pub fn CFMessagePortSetName (ms : CFMessagePortRef , newName : CFStringRef) -> Boolean ; pub fn CFMessagePortInvalidate (ms : CFMessagePortRef) ; pub fn CFMessagePortSendRequest (remote : CFMessagePortRef , msgid : i32 , data : CFDataRef , sendTimeout : CFTimeInterval , rcvTimeout : CFTimeInterval , replyMode : CFStringRef , returnData : * mut CFDataRef ,) -> i32 ; pub fn CFMessagePortGetContext (ms : CFMessagePortRef , context : * mut CFMessagePortContext) ; pub fn CFMessagePortGetInvalidationCallBack (ms : CFMessagePortRef ,) -> CFMessagePortInvalidationCallBack ; pub fn CFMessagePortGetName (ms : CFMessagePortRef) -> CFStringRef ; pub fn CFMessagePortIsRemote (ms : CFMessagePortRef) -> Boolean ; pub fn CFMessagePortIsValid (ms : CFMessagePortRef) -> Boolean ; pub fn CFMessagePortGetTypeID () -> CFTypeID ; }
};
}
