// Generated macro for other_4832 (other)
macro_rules! Depcrate_generatedother_4832 {
() => {
// Module: crate::generated
// Provides: {"other_4832"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `reference` must be a valid pointer."] # [doc = " - `input` must be a valid pointer."] # [doc = " - `output` must be a valid pointer."] # [doc = " - `output_cnt` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOConnectCallAsyncScalarMethod (connection : libc :: mach_port_t , selector : u32 , wake_port : libc :: mach_port_t , reference : * mut u64 , reference_cnt : u32 , input : * const u64 , input_cnt : u32 , output : * mut u64 , output_cnt : * mut u32 ,) -> libc :: kern_return_t ; }
};
}
