// Generated macro for other_264 (other)
macro_rules! Depcrate_filedescriptorother_264 {
() => {
// Module: crate::filedescriptor
// Provides: {"other_264"}
// Dependencies: {}
unsafe extern "C" { pub fn CFFileDescriptorCreate (allocator : CFAllocatorRef , fd : CFFileDescriptorNativeDescriptor , closeOnInvalidate : Boolean , callout : CFFileDescriptorCallBack , context : * const CFFileDescriptorContext ,) -> CFFileDescriptorRef ; pub fn CFFileDescriptorGetNativeDescriptor (f : CFFileDescriptorRef ,) -> CFFileDescriptorNativeDescriptor ; pub fn CFFileDescriptorIsValid (f : CFFileDescriptorRef) -> Boolean ; pub fn CFFileDescriptorGetContext (f : CFFileDescriptorRef , context : * mut CFFileDescriptorContext ,) ; pub fn CFFileDescriptorInvalidate (f : CFFileDescriptorRef) ; pub fn CFFileDescriptorEnableCallBacks (f : CFFileDescriptorRef , callBackTypes : CFOptionFlags) ; pub fn CFFileDescriptorDisableCallBacks (f : CFFileDescriptorRef , callBackTypes : CFOptionFlags) ; pub fn CFFileDescriptorCreateRunLoopSource (allocator : CFAllocatorRef , f : CFFileDescriptorRef , order : CFIndex ,) -> CFRunLoopSourceRef ; pub fn CFFileDescriptorGetTypeID () -> CFTypeID ; }
};
}
