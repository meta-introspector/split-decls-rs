// Generated macro for other_4891 (other)
macro_rules! Depcrate_generatedother_4891 {
() => {
// Module: crate::generated
// Provides: {"other_4891"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Wait for an incoming dataAvailable message on the given notifyPort."] # [doc = ""] # [doc = " This method will simply wait for an incoming message on the given notifyPort.  Once it is received, the return from mach_msg() is returned."] # [doc = ""] # [doc = " Parameter `dataQueue`: The IODataQueueMemory region mapped from the kernel."] # [doc = ""] # [doc = " Parameter `notificationPort`: Mach port on which to listen for incoming messages."] # [doc = ""] # [doc = " Returns: Returns kIOReturnSuccess on success.  Returns kIOReturnBadArgument if either dataQueue is 0 (NULL) or notifyPort is MACH_PORT_NULL.  Returns the result of the mach_msg() listen call on the given port."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data_queue` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IODataQueueWaitForAvailableData (data_queue : * mut IODataQueueMemory , notification_port : libc :: mach_port_t ,) -> IOReturn ; }
};
}
