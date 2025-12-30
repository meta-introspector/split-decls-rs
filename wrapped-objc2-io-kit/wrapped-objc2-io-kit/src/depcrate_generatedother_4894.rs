// Generated macro for other_4894 (other)
macro_rules! Depcrate_generatedother_4894 {
() => {
// Module: crate::generated
// Provides: {"other_4894"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Creates a simple mach message targeting the mach port specified in port."] # [doc = ""] # [doc = " This message is sent when data is added to an empty queue.  It is to notify another user process that new data has become available."] # [doc = " <b>"] # [doc = " Please note that using this method without mapped memory create from an IOSharedDataQueue will result in undefined behavior."] # [doc = " </b>"] # [doc = ""] # [doc = " Parameter `dataQueue`: The IODataQueueMemory region mapped from the kernel created from an IOSharedDataQueue."] # [doc = ""] # [doc = " Parameter `notifyPort`: The mach port to target with the notification message."] # [doc = ""] # [doc = " Returns: Returns kIOReturnSuccess on success.  Returns kIOReturnBadArgument if either dataQueue is 0 (NULL)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `data_queue` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IODataQueueSetNotificationPort (data_queue : * mut IODataQueueMemory , notify_port : libc :: mach_port_t ,) -> IOReturn ; }
};
}
