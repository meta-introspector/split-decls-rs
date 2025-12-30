// Generated macro for other_4829 (other)
macro_rules! Depcrate_generatedother_4829 {
() => {
// Module: crate::generated
// Provides: {"other_4829"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `input_struct` must be a valid pointer."] # [doc = " - `output_struct` must be a valid pointer."] # [doc = " - `output_struct_cnt` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOConnectCallStructMethod (connection : libc :: mach_port_t , selector : u32 , input_struct : * const c_void , input_struct_cnt : usize , output_struct : * mut c_void , output_struct_cnt : * mut usize ,) -> libc :: kern_return_t ; }
};
}
