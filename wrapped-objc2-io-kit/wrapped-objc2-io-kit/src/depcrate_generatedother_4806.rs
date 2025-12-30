// Generated macro for other_4806 (other)
macro_rules! Depcrate_generatedother_4806 {
() => {
// Module: crate::generated
// Provides: {"other_4806"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `notification_type` must be a valid pointer."] # [doc = " - `matching` generic must be of the correct type."] # [doc = " - `matching` generic must be of the correct type."] # [doc = " - `matching` might not allow `None`."] # [doc = " - `notification` must be a valid pointer."] # [cfg (feature = "libc")] # [deprecated] pub fn IOServiceAddNotification (main_port : libc :: mach_port_t , notification_type : * mut io_name_t , matching : Option < & CFDictionary > , wake_port : libc :: mach_port_t , reference : usize , notification : * mut io_iterator_t ,) -> libc :: kern_return_t ; }
};
}
