// Generated macro for other_4809 (other)
macro_rules! Depcrate_generatedother_4809 {
() => {
// Module: crate::generated
// Provides: {"other_4809"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns the busyState of an IOService."] # [doc = ""] # [doc = " Many activities in IOService are asynchronous. When registration, matching, or termination is in progress on an IOService, its busyState is increased by one. Change in busyState to or from zero also changes the IOService's provider's busyState by one, which means that an IOService is marked busy when any of the above activities is occurring on it or any of its clients."] # [doc = ""] # [doc = " Parameter `service`: The IOService whose busyState to return."] # [doc = ""] # [doc = " Parameter `busyState`: The busyState count is returned."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `busy_state` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOServiceGetBusyState (service : io_service_t , busy_state : * mut u32 ,) -> libc :: kern_return_t ; }
};
}
