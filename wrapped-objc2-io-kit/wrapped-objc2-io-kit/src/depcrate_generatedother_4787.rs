// Generated macro for other_4787 (other)
macro_rules! Depcrate_generatedother_4787 {
() => {
// Module: crate::generated
// Provides: {"other_4787"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns the mach port used to initiate communication with IOKit."] # [doc = ""] # [doc = " Functions that don't specify an existing object require the IOKit main port to be passed. This function obtains that port."] # [doc = ""] # [doc = " Parameter `bootstrapPort`: Pass MACH_PORT_NULL for the default."] # [doc = ""] # [doc = " Parameter `mainPort`: The main port is returned."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `main_port` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOMainPort (bootstrap_port : libc :: mach_port_t , main_port : * mut libc :: mach_port_t ,) -> libc :: kern_return_t ; }
};
}
