// Generated macro for other_295 (other)
macro_rules! Depcrate_mach_portother_295 {
() => {
// Module: crate::mach_port
// Provides: {"other_295"}
// Dependencies: {}
unsafe extern "C" { pub fn CFMachPortCreate (allocator : CFAllocatorRef , callout : CFMachPortCallBack , context : * mut CFMachPortContext , shouldFreeInfo : * mut Boolean ,) -> CFMachPortRef ; pub fn CFMachPortCreateWithPort (allocator : CFAllocatorRef , portNum : mach_port_t , callout : CFMachPortCallBack , context : * mut CFMachPortContext , shouldFreeInfo : * mut Boolean ,) -> CFMachPortRef ; pub fn CFMachPortInvalidate (port : CFMachPortRef) ; pub fn CFMachPortCreateRunLoopSource (allocator : CFAllocatorRef , port : CFMachPortRef , order : CFIndex ,) -> CFRunLoopSourceRef ; pub fn CFMachPortSetInvalidationCallBack (port : CFMachPortRef , callout : CFMachPortInvalidationCallBack ,) ; pub fn CFMachPortGetContext (port : CFMachPortRef , context : * mut CFMachPortContext) ; pub fn CFMachPortGetInvalidationCallBack (port : CFMachPortRef) -> CFMachPortInvalidationCallBack ; pub fn CFMachPortGetPort (port : CFMachPortRef) -> mach_port_t ; pub fn CFMachPortIsValid (port : CFMachPortRef) -> Boolean ; pub fn CFMachPortGetTypeID () -> CFTypeID ; }
};
}
