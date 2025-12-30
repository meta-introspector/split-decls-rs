// Generated macro for other_4791 (other)
macro_rules! Depcrate_generatedother_4791 {
() => {
// Module: crate::generated
// Provides: {"other_4791"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Creates and returns a mach port suitable for receiving IOKit messages of the specified type."] # [doc = ""] # [doc = " In the future IOKit may use specialized messages and ports"] # [doc = " instead of the standard ports created by mach_port_allocate(). Use this"] # [doc = " function instead of mach_port_allocate() to ensure compatibility with future"] # [doc = " revisions of IOKit."] # [doc = ""] # [doc = " Parameter `msgType`: Type of message to be sent to this port"] # [doc = " (kOSNotificationMessageID or kOSAsyncCompleteMessageID)"] # [doc = ""] # [doc = " Parameter `recvPort`: The created port is returned."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `recv_port` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOCreateReceivePort (msg_type : u32 , recv_port : * mut libc :: mach_port_t ,) -> libc :: kern_return_t ; }
};
}
