// Generated macro for IODataQueueAllocateNotificationPort (function)
macro_rules! Depcrate_generatedIODataQueueAllocateNotificationPort {
() => {
// Module: crate::generated
// Provides: {"IODataQueueAllocateNotificationPort"}
// Dependencies: {}
# [doc = " Allocates and returns a new mach port able to receive data available notifications from an IODataQueue."] # [doc = ""] # [doc = " This port is intended to be passed down into the kernel and into an IODataQueue to allow it to send the appropriate notification.  The returned mach port is allocated with a queue limit of one message.  This allows only one mach message to be queued up at a time.  The IODataQueue code is written with the restriction in mind and will only queue up a message if no messages alread have been sent."] # [doc = ""] # [doc = " Returns: Returns a newly allocated mach port on success.  On failure, it returns MACH_PORT_NULL."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IODataQueueAllocateNotificationPort () -> libc :: mach_port_t { extern "C-unwind" { fn IODataQueueAllocateNotificationPort () -> libc :: mach_port_t ; } unsafe { IODataQueueAllocateNotificationPort () } }
};
}
