// Generated macro for other_4828 (other)
macro_rules! Depcrate_generatedother_4828 {
() => {
// Module: crate::generated
// Provides: {"other_4828"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `reference` must be a valid pointer."] # [doc = " - `input` must be a valid pointer."] # [doc = " - `input_struct` must be a valid pointer."] # [doc = " - `output` must be a valid pointer."] # [doc = " - `output_cnt` must be a valid pointer."] # [doc = " - `output_struct` must be a valid pointer."] # [doc = " - `output_struct_cnt` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOConnectCallAsyncMethod (connection : libc :: mach_port_t , selector : u32 , wake_port : libc :: mach_port_t , reference : * mut u64 , reference_cnt : u32 , input : * const u64 , input_cnt : u32 , input_struct : * const c_void , input_struct_cnt : usize , output : * mut u64 , output_cnt : * mut u32 , output_struct : * mut c_void , output_struct_cnt : * mut usize ,) -> libc :: kern_return_t ; }
};
}
