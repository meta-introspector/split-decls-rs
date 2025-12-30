// Generated macro for IOConnectSetNotificationPort (function)
macro_rules! Depcrate_generatedIOConnectSetNotificationPort {
() => {
// Module: crate::generated
// Provides: {"IOConnectSetNotificationPort"}
// Dependencies: {}
# [doc = " Set a port to receive family specific notifications."] # [doc = ""] # [doc = " This is a generic method to pass a mach port send right to be be used by family specific notifications."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Parameter `type`: The type of notification requested, not interpreted by IOKit and family defined."] # [doc = ""] # [doc = " Parameter `port`: The port to which to send notifications."] # [doc = ""] # [doc = " Parameter `reference`: Some families may support passing a reference parameter for the callers use with the notification."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOConnectSetNotificationPort (connect : io_connect_t , r#type : u32 , port : libc :: mach_port_t , reference : usize ,) -> libc :: kern_return_t { extern "C-unwind" { fn IOConnectSetNotificationPort (connect : io_connect_t , r#type : u32 , port : libc :: mach_port_t , reference : usize ,) -> libc :: kern_return_t ; } unsafe { IOConnectSetNotificationPort (connect , r#type , port , reference) } }
};
}
