// Generated macro for other_4789 (other)
macro_rules! Depcrate_generatedother_4789 {
() => {
// Module: crate::generated
// Provides: {"other_4789"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Deprecated name for IOMainPort()."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `main_port` must be a valid pointer."] # [cfg (feature = "libc")] # [deprecated] pub fn IOMasterPort (bootstrap_port : libc :: mach_port_t , main_port : * mut libc :: mach_port_t ,) -> libc :: kern_return_t ; }
};
}
