// Generated macro for IONotificationPortCreate (function)
macro_rules! Depcrate_generatedIONotificationPortCreate {
() => {
// Module: crate::generated
// Provides: {"IONotificationPortCreate"}
// Dependencies: {}
# [cfg (feature = "libc")] # [deprecated = "renamed to `IONotificationPort::create`"] # [inline] pub extern "C-unwind" fn IONotificationPortCreate (main_port : libc :: mach_port_t ,) -> IONotificationPortRef { extern "C-unwind" { fn IONotificationPortCreate (main_port : libc :: mach_port_t) -> IONotificationPortRef ; } unsafe { IONotificationPortCreate (main_port) } }
};
}
