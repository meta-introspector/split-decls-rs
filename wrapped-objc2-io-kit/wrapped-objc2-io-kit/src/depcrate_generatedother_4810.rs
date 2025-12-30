// Generated macro for other_4810 (other)
macro_rules! Depcrate_generatedother_4810 {
() => {
// Module: crate::generated
// Provides: {"other_4810"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns the busyState of all IOServices."] # [doc = ""] # [doc = " Many activities in IOService are asynchronous. When registration, matching, or termination is in progress on an IOService, its busyState is increased by one. Change in busyState to or from zero also changes the IOService's provider's busyState by one, which means that an IOService is marked busy when any of the above activities is occurring on it or any of its clients. IOKitGetBusyState returns the busy state of the root of the service plane which reflects the busy state of all IOServices."] # [doc = ""] # [doc = " Parameter `mainPort`: The main port obtained from IOMainPort(). Pass kIOMainPortDefault to look up the default main port."] # [doc = ""] # [doc = " Parameter `busyState`: The busyState count is returned."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `busy_state` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOKitGetBusyState (main_port : libc :: mach_port_t , busy_state : * mut u32 ,) -> libc :: kern_return_t ; }
};
}
