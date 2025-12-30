// Generated macro for other_4830 (other)
macro_rules! Depcrate_generatedother_4830 {
() => {
// Module: crate::generated
// Provides: {"other_4830"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `reference` must be a valid pointer."] # [doc = " - `input_struct` must be a valid pointer."] # [doc = " - `output_struct` must be a valid pointer."] # [doc = " - `output_struct_cnt` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOConnectCallAsyncStructMethod (connection : libc :: mach_port_t , selector : u32 , wake_port : libc :: mach_port_t , reference : * mut u64 , reference_cnt : u32 , input_struct : * const c_void , input_struct_cnt : usize , output_struct : * mut c_void , output_struct_cnt : * mut usize ,) -> libc :: kern_return_t ; }
};
}
